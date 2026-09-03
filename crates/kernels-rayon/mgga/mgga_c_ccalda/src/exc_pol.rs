//! MGGA_C_CCALDA exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ccalda.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_ccalda_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c = f64x8::splat(param_c);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = f64x8::splat(1.0) + param_c;
            let t3 = (simd::cbrt(v_rho0));
            let t4 = t3 * t3;
            let t6 = f64x8::splat(1.0) / t4 / v_rho0;
            let t7 = v_tau0 * t6;
            let t8 = v_rho0 - v_rho1;
            let t9 = v_rho0 + v_rho1;
            let t10 = f64x8::splat(1.0) / t9;
            let t11 = t8 * t10;
            let t12 = f64x8::splat(1.0) + t11;
            let t13 = t12 / f64x8::splat(2.0);
            let t14 = (simd::cbrt(t13));
            let t15 = t14 * t14;
            let t16 = t15 * t13;
            let t17 = t7 * t16;
            let t18 = (simd::cbrt(v_rho1));
            let t19 = t18 * t18;
            let t21 = f64x8::splat(1.0) / t19 / v_rho1;
            let t22 = v_tau1 * t21;
            let t23 = f64x8::splat(1.0) - t11;
            let t24 = t23 / f64x8::splat(2.0);
            let t25 = (simd::cbrt(t24));
            let t26 = t25 * t25;
            let t27 = t26 * t24;
            let t28 = t22 * t27;
            let t30 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t31 = t9 * t9;
            let t32 = (simd::cbrt(t9));
            let t33 = t32 * t32;
            let t35 = f64x8::splat(1.0) / t33 / t31;
            let t38 = t17 + t28 - t30 * t35 / f64x8::splat(8.0);
            let t39 = t2 * t38;
            let t40 = f64x8::splat(M_CBRT6);
            let t41 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t42 = (simd::cbrt(t41));
            let t43 = t42 * t42;
            let t44 = f64x8::splat(1.0) / t43;
            let t45 = t40 * t44;
            let t46 = t39 * t45;
            let t47 = f64x8::splat(M_CBRT2);
            let t48 = t47 * t47;
            let t50 = t45 * t48;
            let t53 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c * t38 * t50;
            let t54 = f64x8::splat(1.0) / t53;
            let t55 = t48 * t54;
            let t56 = t31 * t31;
            let t58 = f64x8::splat(1.0) / t33 / t56;
            let t59 = t30 * t58;
            let t60 = t17 + t28;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t8 * t8;
            let t63 = t61 * t62;
            let t66 = f64x8::splat(1.0) - t59 * t63 / f64x8::splat(8.0);
            let t67 = f64x8::splat(M_CBRT3);
            let t68 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t69 = (simd::cbrt(t68));
            let t70 = t67 * t69;
            let t71 = f64x8::splat(M_CBRT4);
            let t72 = t71 * t71;
            let t75 = t70 * t72 / t32;
            let t77 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t75;
            let t78 = ((t75).sqrt());
            let t81 = ((t75) * (t75).sqrt());
            let t83 = t67 * t67;
            let t84 = t69 * t69;
            let t85 = t83 * t84;
            let t88 = t85 * t71 / t33;
            let t90 = f64x8::splat(3.79785) * t78 + f64x8::splat(0.8969) * t75 + f64x8::splat(0.204775) * t81 + f64x8::splat(0.123235) * t88;
            let t93 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t90;
            let t94 = (simd::ln(t93));
            let t96 = f64x8::splat(0.0621814) * t77 * t94;
            let t97 = t62 * t62;
            let t98 = f64x8::splat(1.0) / t56;
            let t99 = t97 * t98;
            let t100 = (t12).simd_le(zeta_threshold);
            let t101 = (simd::cbrt(zeta_threshold));
            let t102 = t101 * zeta_threshold;
            let t103 = (simd::cbrt(t12));
            let t105 = ((t100).select(t102, t103 * t12));
            let t106 = (t23).simd_le(zeta_threshold);
            let t107 = (simd::cbrt(t23));
            let t109 = ((t106).select(t102, t107 * t23));
            let t110 = t105 + t109 - f64x8::splat(2.0);
            let t113 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t47 - f64x8::splat(2.0));
            let t114 = t110 * t113;
            let t116 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t75;
            let t121 = f64x8::splat(7.05945) * t78 + f64x8::splat(1.549425) * t75 + f64x8::splat(0.420775) * t81 + f64x8::splat(0.1562925) * t88;
            let t124 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t121;
            let t125 = (simd::ln(t124));
            let t129 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t75;
            let t134 = f64x8::splat(5.1785) * t78 + f64x8::splat(0.905775) * t75 + f64x8::splat(0.1100325) * t81 + f64x8::splat(0.1241775) * t88;
            let t137 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t134;
            let t138 = (simd::ln(t137));
            let t139 = t129 * t138;
            let t141 = -f64x8::splat(0.0310907) * t116 * t125 + t96 - f64x8::splat(0.0197516734986138) * t139;
            let t142 = t114 * t141;
            let t146 = -t96 + t99 * t142 + f64x8::splat(0.0197516734986138) * t114 * t139;
            let t147 = t66 * t146;
            let t148 = t55 * t147;
            let t150 = f64x8::splat(5.0) / f64x8::splat(9.0) * t46 * t148;
            let t151 = t39 * t40;
            let t152 = t44 * t48;
            let t153 = t152 * t54;
            let t154 = t151 * t153;
            let t156 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t154;
            let t157 = t156 * t146;
            let tzk0 = t150 + t157;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
