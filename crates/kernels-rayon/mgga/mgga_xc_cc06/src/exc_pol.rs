//! MGGA_XC_CC06 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`
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
pub fn mgga_xc_cc06_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = v_rho0 * t8;
            let t11 = (f64x8::splat(2.0) * t9).simd_le(zeta_threshold);
            let t12 = (simd::cbrt(zeta_threshold));
            let t13 = t12 * zeta_threshold;
            let t14 = f64x8::splat(M_CBRT2);
            let t15 = t14 * v_rho0;
            let t16 = (simd::cbrt(t9));
            let t20 = ((t11).select(t13, f64x8::splat(2.0) * t15 * t8 * t16));
            let t21 = (simd::cbrt(t7));
            let t25 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t20 * t21));
            let t26 = (v_rho1).simd_le(dens_threshold);
            let t27 = v_rho1 * t8;
            let t29 = (f64x8::splat(2.0) * t27).simd_le(zeta_threshold);
            let t30 = t14 * v_rho1;
            let t31 = (simd::cbrt(t27));
            let t35 = ((t29).select(t13, f64x8::splat(2.0) * t30 * t8 * t31));
            let t39 = ((t26).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t35 * t21));
            let t40 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t41 = (simd::cbrt(t40));
            let t42 = t3 * t41;
            let t43 = f64x8::splat(M_CBRT4);
            let t44 = t43 * t43;
            let t47 = t42 * t44 / t21;
            let t49 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t47;
            let t50 = ((t47).sqrt());
            let t53 = ((t47) * (t47).sqrt());
            let t55 = t3 * t3;
            let t56 = t41 * t41;
            let t57 = t55 * t56;
            let t58 = t21 * t21;
            let t59 = f64x8::splat(1.0) / t58;
            let t61 = t57 * t43 * t59;
            let t63 = f64x8::splat(3.79785) * t50 + f64x8::splat(0.8969) * t47 + f64x8::splat(0.204775) * t53 + f64x8::splat(0.123235) * t61;
            let t66 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t63;
            let t67 = (simd::ln(t66));
            let t69 = f64x8::splat(0.062182) * t49 * t67;
            let t70 = v_rho0 - v_rho1;
            let t71 = t70 * t70;
            let t72 = t71 * t71;
            let t73 = t7 * t7;
            let t74 = t73 * t73;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t72 * t75;
            let t77 = t70 * t8;
            let t78 = f64x8::splat(1.0) + t77;
            let t79 = (t78).simd_le(zeta_threshold);
            let t80 = (simd::cbrt(t78));
            let t82 = ((t79).select(t13, t80 * t78));
            let t83 = f64x8::splat(1.0) - t77;
            let t84 = (t83).simd_le(zeta_threshold);
            let t85 = (simd::cbrt(t83));
            let t87 = ((t84).select(t13, t85 * t83));
            let t88 = t82 + t87 - f64x8::splat(2.0);
            let t91 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t14 - f64x8::splat(2.0));
            let t92 = t88 * t91;
            let t94 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t47;
            let t99 = f64x8::splat(7.05945) * t50 + f64x8::splat(1.549425) * t47 + f64x8::splat(0.420775) * t53 + f64x8::splat(0.1562925) * t61;
            let t102 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t99;
            let t103 = (simd::ln(t102));
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t47;
            let t112 = f64x8::splat(5.1785) * t50 + f64x8::splat(0.905775) * t47 + f64x8::splat(0.1100325) * t53 + f64x8::splat(0.1241775) * t61;
            let t115 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t112;
            let t116 = (simd::ln(t115));
            let t117 = t107 * t116;
            let t119 = -f64x8::splat(0.03109) * t94 * t103 + t69 - f64x8::splat(0.019751789702565206) * t117;
            let t120 = t92 * t119;
            let t124 = t25 + t39 - t69 + t76 * t120 + f64x8::splat(0.019751789702565206) * t92 * t117;
            let t125 = t55 * t43;
            let t126 = (simd::cbrt(v_rho0));
            let t127 = t126 * t126;
            let t129 = f64x8::splat(1.0) / t127 / v_rho0;
            let t130 = v_lapl0 * t129;
            let t131 = t78 / f64x8::splat(2.0);
            let t132 = (simd::cbrt(t131));
            let t133 = t132 * t132;
            let t134 = t133 * t131;
            let t136 = (simd::cbrt(v_rho1));
            let t137 = t136 * t136;
            let t139 = f64x8::splat(1.0) / t137 / v_rho1;
            let t140 = v_lapl1 * t139;
            let t141 = t83 / f64x8::splat(2.0);
            let t142 = (simd::cbrt(t141));
            let t143 = t142 * t142;
            let t144 = t143 * t141;
            let t148 = t125 * t56 * (t130 * t134 + t140 * t144);
            let t150 = -f64x8::splat(0.0007) + f64x8::splat(0.002) * t148;
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.0065) * t148;
            let t153 = f64x8::splat(1.0) / t152;
            let t155 = t150 * t153 + f64x8::splat(1.0);
            let tzk0 = t124 * t155;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
