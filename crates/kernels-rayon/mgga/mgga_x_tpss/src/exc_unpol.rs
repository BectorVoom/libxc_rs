//! MGGA_X_TPSS exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

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
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = 1.0 / rho[ip];
        let t23 = 1.0 / tau[ip];
        let t25 = sigma[ip] * t21 * t23 / 8.0;
        let t26 = param_BLOC_b * sigma[ip];
        let t30 = param_BLOC_a + t26 * t21 * t23 / 8.0;
        let t31 = f64::powf(t25, t30);
        let t32 = param_c * t31;
        let t33 = sigma[ip] * sigma[ip];
        let t34 = rho[ip] * rho[ip];
        let t35 = 1.0 / t34;
        let t36 = t33 * t35;
        let t37 = tau[ip] * tau[ip];
        let t38 = 1.0 / t37;
        let t39 = t36 * t38;
        let t41 = 1.0 + t39 / 64.0;
        let t42 = t41 * t41;
        let t43 = 1.0 / t42;
        let t46 = M_CBRT6;
        let t47 = (10.0 / 81.0 + t32 * t43) * t46;
        let t48 = M_PI * M_PI;
        let t49 = pow_1_3(t48);
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t52 = t47 * t51;
        let t53 = M_CBRT2;
        let t54 = t53 * t53;
        let t55 = sigma[ip] * t54;
        let t56 = t19 * t19;
        let t58 = 1.0 / t56 / t34;
        let t59 = t55 * t58;
        let t62 = tau[ip] * t54;
        let t64 = 1.0 / t56 / rho[ip];
        let t67 = t62 * t64 - t59 / 8.0;
        let t71 = 5.0 / 9.0 * t67 * t46 * t51 - 1.0;
        let t72 = param_b * t67;
        let t73 = t46 * t51;
        let t74 = t73 * t71;
        let t77 = 5.0 * t72 * t74 + 9.0;
        let t78 = f64::sqrt(t77);
        let t79 = 1.0 / t78;
        let t84 = 27.0 / 20.0 * t71 * t79 + t73 * t59 / 36.0;
        let t85 = t84 * t84;
        let t88 = t46 * t46;
        let t90 = 1.0 / t49 / t48;
        let t91 = t88 * t90;
        let t92 = t33 * t53;
        let t93 = t34 * t34;
        let t94 = t93 * rho[ip];
        let t96 = 1.0 / t19 / t94;
        let t97 = t92 * t96;
        let t100 = 100.0 * t91 * t97 + 162.0 * t39;
        let t101 = f64::sqrt(t100);
        let t105 = 1.0 / param_kappa * t88;
        let t106 = t105 * t90;
        let t109 = f64::sqrt(param_e);
        let t110 = t109 * t33;
        let t111 = t35 * t38;
        let t114 = param_e * param_mu;
        let t115 = t48 * t48;
        let t116 = 1.0 / t115;
        let t117 = t33 * sigma[ip];
        let t118 = t116 * t117;
        let t119 = t93 * t93;
        let t120 = 1.0 / t119;
        let t124 = t52 * t59 / 24.0 + 146.0 / 2025.0 * t85 - 73.0 / 97200.0 * t84 * t101 + 25.0 / 472392.0 * t106 * t97 + t110 * t111 / 720.0 + t114 * t118 * t120 / 576.0;
        let t125 = t109 * t46;
        let t129 = 1.0 + t125 * t51 * t59 / 24.0;
        let t130 = t129 * t129;
        let t131 = 1.0 / t130;
        let t133 = t124 * t131 + param_kappa;
        let t138 = 1.0 + param_kappa * (1.0 - param_kappa / t133);
        let t142 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t138);
        let tzk0 = 2.0 * t142;
        zk[ip] += tzk0;
    }
}
