//! GGA_X_HTBS exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 116 shared lines across all orders.
//! Delta: 116 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_htbs_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (116 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t33 = t29 / t31;
        let t34 = f64::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t39 = t33 * t34 * t37;
        let t40 = t39 / 12.0;
        let t41 = t40 <= 0.6e0;
        let t42 = t31 * t31;
        let t43 = 1.0 / t42;
        let t44 = t28 * t43;
        let t45 = rho0 * rho0;
        let t46 = t35 * t35;
        let t48 = 1.0 / t46 / t45;
        let t49 = sigma0 * t48;
        let t50 = t44 * t49;
        let t53 = f64::exp(-t50 / 24.0);
        let t58 = 1.0 / t31 / t30;
        let t59 = t29 * t58;
        let t60 = sigma0 * sigma0;
        let t61 = t45 * t45;
        let t62 = t61 * rho0;
        let t64 = 1.0 / t35 / t62;
        let t66 = t59 * t60 * t64;
        let t68 = 1.0 + 0.13780328706878157639e-4 * t66;
        let t69 = f64::ln(t68);
        let t70 = 0.804e0 + 5.0 / 972.0 * t50 + 0.4002424276710846245e-2 * t44 * t49 * t53 + t69;
        let t73 = 0.1804e1 - 0.646416e0 / t70;
        let t74 = 0.26e1 <= t40;
        let t76 = f64::exp(-0.1137619054542480583e-1 * t50);
        let t78 = 0.1804e1 - 0.804e0 * t76;
        let t79 = 0.190125e0 * t39;
        let t80 = 0.195e0 * t50;
        let t81 = t34 * sigma0;
        let t82 = 1.0 / t61;
        let t84 = 0.88128321188908374119e-2 * t81 * t82;
        let t85 = 0.26041666666666666667e-2 * t66;
        let t88 = t28 / t42 / t30;
        let t89 = t34 * t60;
        let t90 = t61 * t45;
        let t92 = 1.0 / t46 / t90;
        let t95 = 0.16276041666666666667e-3 * t88 * t89 * t92;
        let t96 = -0.40608e0 + t79 - t80 + t84 - t85 + t95;
        let t98 = 0.140608e1 - t79 + t80 - t84 + t85 - t95;
        let t101 = piecewise5(t41, t73, t74, t78, t98 * t73 + t96 * t78);
        let t105 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t101);
        let t106 = rho1 <= dens_threshold;
        let t107 = -t16;
        let t109 = piecewise5(t14, t11, t10, t15, t107 * t7);
        let t110 = 1.0 + t109;
        let t111 = t110 <= zeta_threshold;
        let t112 = pow_1_3(t110);
        let t114 = piecewise3(t111, t22, t112 * t110);
        let t115 = t114 * t26;
        let t116 = f64::sqrt(sigma2);
        let t117 = pow_1_3(rho1);
        let t119 = 1.0 / t117 / rho1;
        let t121 = t33 * t116 * t119;
        let t122 = t121 / 12.0;
        let t123 = t122 <= 0.6e0;
        let t124 = rho1 * rho1;
        let t125 = t117 * t117;
        let t127 = 1.0 / t125 / t124;
        let t128 = sigma2 * t127;
        let t129 = t44 * t128;
        let t132 = f64::exp(-t129 / 24.0);
        let t136 = sigma2 * sigma2;
        let t137 = t124 * t124;
        let t138 = t137 * rho1;
        let t140 = 1.0 / t117 / t138;
        let t142 = t59 * t136 * t140;
        let t144 = 1.0 + 0.13780328706878157639e-4 * t142;
        let t145 = f64::ln(t144);
        let t146 = 0.804e0 + 5.0 / 972.0 * t129 + 0.4002424276710846245e-2 * t44 * t128 * t132 + t145;
        let t149 = 0.1804e1 - 0.646416e0 / t146;
        let t150 = 0.26e1 <= t122;
        let t152 = f64::exp(-0.1137619054542480583e-1 * t129);
        let t154 = 0.1804e1 - 0.804e0 * t152;
        let t155 = 0.190125e0 * t121;
        let t156 = 0.195e0 * t129;
        let t157 = t116 * sigma2;
        let t158 = 1.0 / t137;
        let t160 = 0.88128321188908374119e-2 * t157 * t158;
        let t161 = 0.26041666666666666667e-2 * t142;
        let t162 = t116 * t136;
        let t163 = t137 * t124;
        let t165 = 1.0 / t125 / t163;
        let t168 = 0.16276041666666666667e-3 * t88 * t162 * t165;
        let t169 = -0.40608e0 + t155 - t156 + t160 - t161 + t168;
        let t171 = 0.140608e1 - t155 + t156 - t160 + t161 - t168;
        let t174 = piecewise5(t123, t149, t150, t154, t171 * t149 + t169 * t154);
        let t178 = piecewise3(t106, 0.0, -3.0 / 8.0 * t5 * t115 * t174);
        let tzk0 = t105 + t178;
        zk[ip] += tzk0;
    }
}
