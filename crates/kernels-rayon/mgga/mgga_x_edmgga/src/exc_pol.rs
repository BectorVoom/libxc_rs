//! MGGA_X_EDMGGA exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_edmgga.c`
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
pub fn mgga_x_edmgga_exc_pol(
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
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(M_CBRT4);
            let t30 = t3 * t3;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t35 = t29 * t30 * t33 / f64x8::splat(9.0);
            let t36 = f64x8::splat(1.0) - t35;
            let t37 = (simd::cbrt(v_rho0));
            let t38 = t37 * t37;
            let t40 = f64x8::splat(1.0) / t38 / v_rho0;
            let t42 = v_rho0 * v_rho0;
            let t44 = f64x8::splat(1.0) / t38 / t42;
            let t50 = f64x8::splat(M_CBRT6);
            let t52 = t33 * t33;
            let t53 = f64x8::splat(1.0) / t52;
            let t54 = (v_tau0 * t40 - v_sigma0 * t44 / f64x8::splat(8.0) - v_lapl0 * t40 / f64x8::splat(4.0)) * t50 * t53;
            let t55 = f64x8::splat(5.0) / f64x8::splat(9.0) * t54;
            let t56 = (-t55).simd_lt(-f64x8::splat(14205.545454545454));
            let t57 = f64x8::splat(0.39111111111111113) * t54;
            let t59 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7041420454545455) - t57);
            let t61 = ((t59).select(-f64x8::splat(0.00014204545454545454), f64x8::splat(0.704) - t57));
            let t64 = t61 * t61;
            let t65 = t64 * t61;
            let t66 = f64x8::splat(1.0) / t65;
            let t69 = f64x8::splat(1.0) - t55;
            let t70 = t69 * t69;
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.495616) * t70;
            let t73 = ((t72).sqrt());
            let t75 = ((t56).select(-f64x8::splat(1.0) / t61 / f64x8::splat(2.0) + t66 / f64x8::splat(8.0), f64x8::splat(0.704) - t57 + t73));
            let t76 = t36 * t75;
            let t77 = ((f64x8::splat(30.0)).sqrt());
            let t78 = t36 * t77;
            let t79 = ((t75).sqrt());
            let t80 = t36 * t36;
            let t83 = f64x8::splat(1.0) / t80 / t36 * t77;
            let t85 = f64x8::splat(0.6018478308354863) * t80 - f64x8::splat(0.0206514);
            let t86 = t75 - f64x8::splat(1.0);
            let t90 = (simd::ln(f64x8::splat(0.3910293204892512) * t83 * t85 * t86 + ((((f64x8::splat(0.3910293204892512) * t83 * t85 * t86) * (f64x8::splat(0.3910293204892512) * t83 * t85 * t86)) + f64x8::splat(1.0)).sqrt())));
            let t94 = f64x8::splat(1.0) + f64x8::splat(0.14163895778062927) * t78 * t79 * t90;
            let t95 = f64x8::splat(1.0) / t94;
            let t97 = t76 * t95 + t35;
            let t101 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t97));
            let t102 = (v_rho1).simd_le(dens_threshold);
            let t103 = -t17;
            let t105 = ((t15).select(t12, (t11).select(t16, t103 * t8)));
            let t106 = f64x8::splat(1.0) + t105;
            let t107 = (t106).simd_le(zeta_threshold);
            let t108 = (simd::cbrt(t106));
            let t110 = ((t107).select(t23, t108 * t106));
            let t111 = t110 * t27;
            let t112 = (simd::cbrt(v_rho1));
            let t113 = t112 * t112;
            let t115 = f64x8::splat(1.0) / t113 / v_rho1;
            let t117 = v_rho1 * v_rho1;
            let t119 = f64x8::splat(1.0) / t113 / t117;
            let t126 = (v_tau1 * t115 - v_sigma2 * t119 / f64x8::splat(8.0) - v_lapl1 * t115 / f64x8::splat(4.0)) * t50 * t53;
            let t127 = f64x8::splat(5.0) / f64x8::splat(9.0) * t126;
            let t128 = (-t127).simd_lt(-f64x8::splat(14205.545454545454));
            let t129 = f64x8::splat(0.39111111111111113) * t126;
            let t131 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7041420454545455) - t129);
            let t133 = ((t131).select(-f64x8::splat(0.00014204545454545454), f64x8::splat(0.704) - t129));
            let t136 = t133 * t133;
            let t137 = t136 * t133;
            let t138 = f64x8::splat(1.0) / t137;
            let t141 = f64x8::splat(1.0) - t127;
            let t142 = t141 * t141;
            let t144 = f64x8::splat(1.0) + f64x8::splat(0.495616) * t142;
            let t145 = ((t144).sqrt());
            let t147 = ((t128).select(-f64x8::splat(1.0) / t133 / f64x8::splat(2.0) + t138 / f64x8::splat(8.0), f64x8::splat(0.704) - t129 + t145));
            let t148 = t36 * t147;
            let t149 = ((t147).sqrt());
            let t150 = t147 - f64x8::splat(1.0);
            let t154 = (simd::ln(f64x8::splat(0.3910293204892512) * t83 * t85 * t150 + ((((f64x8::splat(0.3910293204892512) * t83 * t85 * t150) * (f64x8::splat(0.3910293204892512) * t83 * t85 * t150)) + f64x8::splat(1.0)).sqrt())));
            let t158 = f64x8::splat(1.0) + f64x8::splat(0.14163895778062927) * t78 * t149 * t154;
            let t159 = f64x8::splat(1.0) / t158;
            let t161 = t148 * t159 + t35;
            let t165 = ((t102).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t111 * t161));
            let tzk0 = t101 + t165;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
