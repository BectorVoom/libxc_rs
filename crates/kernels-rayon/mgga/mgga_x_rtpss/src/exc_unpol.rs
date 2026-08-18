//! MGGA_X_RTPSS exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rtpss.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rtpss_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let t21 = sigma[ip] * sigma[ip];
        let t22 = param_c * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = 1.0 / t23;
        let t25 = tau[ip] * tau[ip];
        let t26 = 1.0 / t25;
        let t27 = t24 * t26;
        let t28 = t21 * t24;
        let t29 = t28 * t26;
        let t31 = 1.0 + t29 / 64.0;
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t27 * t33;
        let t38 = M_CBRT6;
        let t39 = (10.0 / 81.0 + t22 * t34 / 64.0) * t38;
        let t40 = M_PI * M_PI;
        let t41 = pow_1_3(t40);
        let t42 = t41 * t41;
        let t43 = 1.0 / t42;
        let t44 = t39 * t43;
        let t45 = M_CBRT2;
        let t46 = t45 * t45;
        let t47 = sigma[ip] * t46;
        let t48 = t19 * t19;
        let t50 = 1.0 / t48 / t23;
        let t51 = t47 * t50;
        let t54 = tau[ip] * t46;
        let t56 = 1.0 / t48 / rho[ip];
        let t59 = t54 * t56 - t51 / 8.0;
        let t63 = 5.0 / 9.0 * t59 * t38 * t43 - 1.0;
        let t64 = param_b * t59;
        let t65 = t38 * t43;
        let t66 = t65 * t63;
        let t69 = 5.0 * t64 * t66 + 9.0;
        let t70 = f64::sqrt(t69);
        let t71 = 1.0 / t70;
        let t76 = 27.0 / 20.0 * t63 * t71 + t65 * t51 / 36.0;
        let t77 = t76 * t76;
        let t80 = t38 * t38;
        let t82 = 1.0 / t41 / t40;
        let t83 = t80 * t82;
        let t84 = t21 * t45;
        let t85 = t23 * t23;
        let t86 = t85 * rho[ip];
        let t88 = 1.0 / t19 / t86;
        let t89 = t84 * t88;
        let t92 = 100.0 * t83 * t89 + 162.0 * t29;
        let t93 = f64::sqrt(t92);
        let t96 = 1.0 / param_kappa;
        let t97 = t96 * t80;
        let t98 = t97 * t82;
        let t101 = f64::sqrt(param_e);
        let t102 = t101 * t21;
        let t105 = param_e * param_mu;
        let t106 = t40 * t40;
        let t107 = 1.0 / t106;
        let t108 = t21 * sigma[ip];
        let t109 = t107 * t108;
        let t110 = t85 * t85;
        let t111 = 1.0 / t110;
        let t115 = t44 * t51 / 24.0 + 146.0 / 2025.0 * t77 - 73.0 / 97200.0 * t76 * t93 + 25.0 / 472392.0 * t98 * t89 + t102 * t27 / 720.0 + t105 * t109 * t111 / 576.0;
        let t116 = t101 * t38;
        let t120 = 1.0 + t116 * t43 * t51 / 24.0;
        let t121 = t120 * t120;
        let t122 = 1.0 / t121;
        let t125 = f64::exp(-t115 * t122 * t96);
        let t128 = 1.0 + param_kappa * (1.0 - t125);
        let t132 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t128);
        let tzk0 = 2.0 * t132;
        zk[ip] += tzk0;
    }
}
