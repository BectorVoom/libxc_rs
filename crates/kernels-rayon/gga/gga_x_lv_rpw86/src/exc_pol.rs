//! GGA_X_LV_RPW86 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lv_rpw86.c`
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
pub fn gga_x_lv_rpw86_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
        let mut acc_zk = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t33 = t28 / t31;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t40 = t33 * v_sigma0 * t38;
            let t42 = f64x8::splat(1.0) + f64x8::splat(0.003931018518518519) * t40;
            let t43 = v_sigma0 * v_sigma0;
            let t44 = t43 * v_sigma0;
            let t45 = t34 * t34;
            let t46 = t45 * t45;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t44 * t47;
            let t49 = f64x8::splat(9.704561350131286e-08) * t48;
            let t50 = f64x8::splat(1.0) + t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t54 = t28 * t28;
            let t57 = t54 / t30 / t29;
            let t58 = t45 * v_rho0;
            let t60 = f64x8::splat(1.0) / t35 / t58;
            let t65 = f64x8::splat(1.0) + f64x8::splat(0.077125) * t40 + f64x8::splat(0.030086805555555554) * t57 * t43 * t60 + f64x8::splat(7.26282598747199e-07) * t48;
            let t66 = (simd::pow(t65, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t67 = f64x8::splat(1.15) + t49;
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = t66 * t68;
            let t72 = t42 * t51 + f64x8::splat(9.704561350131286e-08) * t48 * t69;
            let t76 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t72));
            let t77 = (v_rho1).simd_le(dens_threshold);
            let t78 = -t16;
            let t80 = ((t14).select(t11, (t10).select(t15, t78 * t7)));
            let t81 = f64x8::splat(1.0) + t80;
            let t82 = (t81).simd_le(zeta_threshold);
            let t83 = (simd::cbrt(t81));
            let t85 = ((t82).select(t22, t83 * t81));
            let t86 = t85 * t26;
            let t87 = v_rho1 * v_rho1;
            let t88 = (simd::cbrt(v_rho1));
            let t89 = t88 * t88;
            let t91 = f64x8::splat(1.0) / t89 / t87;
            let t93 = t33 * v_sigma2 * t91;
            let t95 = f64x8::splat(1.0) + f64x8::splat(0.003931018518518519) * t93;
            let t96 = v_sigma2 * v_sigma2;
            let t97 = t96 * v_sigma2;
            let t98 = t87 * t87;
            let t99 = t98 * t98;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t97 * t100;
            let t102 = f64x8::splat(9.704561350131286e-08) * t101;
            let t103 = f64x8::splat(1.0) + t102;
            let t104 = f64x8::splat(1.0) / t103;
            let t107 = t98 * v_rho1;
            let t109 = f64x8::splat(1.0) / t88 / t107;
            let t114 = f64x8::splat(1.0) + f64x8::splat(0.077125) * t93 + f64x8::splat(0.030086805555555554) * t57 * t96 * t109 + f64x8::splat(7.26282598747199e-07) * t101;
            let t115 = (simd::pow(t114, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t116 = f64x8::splat(1.15) + t102;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t115 * t117;
            let t121 = t95 * t104 + f64x8::splat(9.704561350131286e-08) * t101 * t118;
            let t125 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t86 * t121));
            let tzk0 = t76 + t125;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
