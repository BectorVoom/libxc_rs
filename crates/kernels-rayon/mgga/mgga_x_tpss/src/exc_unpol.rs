//! MGGA_X_TPSS exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tpss_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_BLOC_a: f64,
    param_BLOC_b: f64,
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_BLOC_a = f64x8::splat(param_BLOC_a);
    let param_BLOC_b = f64x8::splat(param_BLOC_b);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_e = f64x8::splat(param_e);
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(1.0) / v_rho;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t25 = v_sigma * t21 * t23 / f64x8::splat(8.0);
            let t26 = param_BLOC_b * v_sigma;
            let t30 = param_BLOC_a + t26 * t21 * t23 / f64x8::splat(8.0);
            let t31 = (simd::pow(t25, t30));
            let t32 = param_c * t31;
            let t33 = v_sigma * v_sigma;
            let t34 = v_rho * v_rho;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t33 * t35;
            let t37 = v_tau * v_tau;
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = t36 * t38;
            let t41 = f64x8::splat(1.0) + t39 / f64x8::splat(64.0);
            let t42 = t41 * t41;
            let t43 = f64x8::splat(1.0) / t42;
            let t46 = f64x8::splat(M_CBRT6);
            let t47 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t32 * t43) * t46;
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t47 * t51;
            let t53 = f64x8::splat(M_CBRT2);
            let t54 = t53 * t53;
            let t55 = v_sigma * t54;
            let t56 = t19 * t19;
            let t58 = f64x8::splat(1.0) / t56 / t34;
            let t59 = t55 * t58;
            let t62 = v_tau * t54;
            let t64 = f64x8::splat(1.0) / t56 / v_rho;
            let t67 = t62 * t64 - t59 / f64x8::splat(8.0);
            let t71 = f64x8::splat(5.0) / f64x8::splat(9.0) * t67 * t46 * t51 - f64x8::splat(1.0);
            let t72 = param_b * t67;
            let t73 = t46 * t51;
            let t74 = t73 * t71;
            let t77 = f64x8::splat(5.0) * t72 * t74 + f64x8::splat(9.0);
            let t78 = ((t77).sqrt());
            let t79 = f64x8::splat(1.0) / t78;
            let t84 = f64x8::splat(27.0) / f64x8::splat(20.0) * t71 * t79 + t73 * t59 / f64x8::splat(36.0);
            let t85 = t84 * t84;
            let t88 = t46 * t46;
            let t90 = f64x8::splat(1.0) / t49 / t48;
            let t91 = t88 * t90;
            let t92 = t33 * t53;
            let t93 = t34 * t34;
            let t94 = t93 * v_rho;
            let t96 = f64x8::splat(1.0) / t19 / t94;
            let t97 = t92 * t96;
            let t100 = f64x8::splat(100.0) * t91 * t97 + f64x8::splat(162.0) * t39;
            let t101 = ((t100).sqrt());
            let t105 = f64x8::splat(1.0) / param_kappa * t88;
            let t106 = t105 * t90;
            let t109 = ((param_e).sqrt());
            let t110 = t109 * t33;
            let t111 = t35 * t38;
            let t114 = param_e * param_mu;
            let t115 = t48 * t48;
            let t116 = f64x8::splat(1.0) / t115;
            let t117 = t33 * v_sigma;
            let t118 = t116 * t117;
            let t119 = t93 * t93;
            let t120 = f64x8::splat(1.0) / t119;
            let t124 = t52 * t59 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t85 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t84 * t101 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t106 * t97 + t110 * t111 / f64x8::splat(720.0) + t114 * t118 * t120 / f64x8::splat(576.0);
            let t125 = t109 * t46;
            let t129 = f64x8::splat(1.0) + t125 * t51 * t59 / f64x8::splat(24.0);
            let t130 = t129 * t129;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = t124 * t131 + param_kappa;
            let t138 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t133);
            let t142 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t138));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
