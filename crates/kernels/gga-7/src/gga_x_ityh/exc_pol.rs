//! GGA_X_ITYH exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 136 shared lines across all orders.
//! Delta: 136 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_ityh_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_omega_0: f64,
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
        // --- shared preamble (136 lines) ---
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
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = t2 * t2;
        let t29 = M_PI * t28;
        let t30 = 1.0 / M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = M_CBRT4;
        let t34 = t32 * t33;
        let t35 = t28 * t32;
        let t36 = t35 * t33;
        let t37 = rho0 * rho0;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t41 = 1.0 / t39 / t37;
        let t42 = sigma0 * t41;
        let t43 = f64::sqrt(sigma0);
        let t45 = 1.0 / t38 / rho0;
        let t46 = t43 * t45;
        let t47 = f64::ln(t46 + f64::sqrt(t46 * t46 + 1.0));
        let t50 = 1.0 + 0.252e-1 * t46 * t47;
        let t51 = 1.0 / t50;
        let t55 = 1.0 + 0.93333333333333333332e-3 * t36 * t42 * t51;
        let t58 = t29 * t34 / t55;
        let t59 = f64::sqrt(t58);
        let t61 = param_hyb_omega_0 / t59;
        let t62 = M_CBRT2;
        let t63 = t19 * t6;
        let t64 = pow_1_3(t63);
        let t65 = 1.0 / t64;
        let t66 = t62 * t65;
        let t68 = t61 * t66 / 2.0;
        let t69 = 0.135e1 <= t68;
        let t70 = 0.135e1 < t68;
        let t71 = piecewise3(t70, t68, 0.135e1);
        let t72 = t71 * t71;
        let t75 = t72 * t72;
        let t76 = 1.0 / t75;
        let t78 = t75 * t72;
        let t79 = 1.0 / t78;
        let t81 = t75 * t75;
        let t82 = 1.0 / t81;
        let t85 = 1.0 / t81 / t72;
        let t88 = 1.0 / t81 / t75;
        let t91 = 1.0 / t81 / t78;
        let t93 = t81 * t81;
        let t94 = 1.0 / t93;
        let t97 = piecewise3(t70, 0.135e1, t68);
        let t98 = f64::sqrt(M_PI);
        let t99 = 1.0 / t97;
        let t101 = erf_approx(t99 / 2.0);
        let t103 = t97 * t97;
        let t104 = 1.0 / t103;
        let t106 = f64::exp(-t104 / 4.0);
        let t107 = t106 - 1.0;
        let t110 = t106 - 3.0 / 2.0 - 2.0 * t103 * t107;
        let t113 = t98 * t101 + 2.0 * t97 * t110;
        let t117 = piecewise3(t69, 1.0 / t72 / 36.0 - t76 / 960.0 + t79 / 26880.0 - t82 / 829440.0 + t85 / 28385280.0 - t88 / 0.107347968e10 + t91 / 0.445906944e11 - t94 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t97 * t113);
        let t118 = t27 * t117;
        let t119 = t118 * t55;
        let t122 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t119);
        let t123 = rho1 <= dens_threshold;
        let t124 = -t16;
        let t126 = piecewise5(t14, t11, t10, t15, t124 * t7);
        let t127 = 1.0 + t126;
        let t128 = t127 <= zeta_threshold;
        let t129 = pow_1_3(t127);
        let t131 = piecewise3(t128, t22, t129 * t127);
        let t132 = t5 * t131;
        let t133 = rho1 * rho1;
        let t134 = pow_1_3(rho1);
        let t135 = t134 * t134;
        let t137 = 1.0 / t135 / t133;
        let t138 = sigma2 * t137;
        let t139 = f64::sqrt(sigma2);
        let t141 = 1.0 / t134 / rho1;
        let t142 = t139 * t141;
        let t143 = f64::ln(t142 + f64::sqrt(t142 * t142 + 1.0));
        let t146 = 1.0 + 0.252e-1 * t142 * t143;
        let t147 = 1.0 / t146;
        let t151 = 1.0 + 0.93333333333333333332e-3 * t36 * t138 * t147;
        let t154 = t29 * t34 / t151;
        let t155 = f64::sqrt(t154);
        let t157 = param_hyb_omega_0 / t155;
        let t158 = t127 * t6;
        let t159 = pow_1_3(t158);
        let t160 = 1.0 / t159;
        let t161 = t62 * t160;
        let t163 = t157 * t161 / 2.0;
        let t164 = 0.135e1 <= t163;
        let t165 = 0.135e1 < t163;
        let t166 = piecewise3(t165, t163, 0.135e1);
        let t167 = t166 * t166;
        let t170 = t167 * t167;
        let t171 = 1.0 / t170;
        let t173 = t170 * t167;
        let t174 = 1.0 / t173;
        let t176 = t170 * t170;
        let t177 = 1.0 / t176;
        let t180 = 1.0 / t176 / t167;
        let t183 = 1.0 / t176 / t170;
        let t186 = 1.0 / t176 / t173;
        let t188 = t176 * t176;
        let t189 = 1.0 / t188;
        let t192 = piecewise3(t165, 0.135e1, t163);
        let t193 = 1.0 / t192;
        let t195 = erf_approx(t193 / 2.0);
        let t197 = t192 * t192;
        let t198 = 1.0 / t197;
        let t200 = f64::exp(-t198 / 4.0);
        let t201 = t200 - 1.0;
        let t204 = t200 - 3.0 / 2.0 - 2.0 * t197 * t201;
        let t207 = 2.0 * t192 * t204 + t98 * t195;
        let t211 = piecewise3(t164, 1.0 / t167 / 36.0 - t171 / 960.0 + t174 / 26880.0 - t177 / 829440.0 + t180 / 28385280.0 - t183 / 0.107347968e10 + t186 / 0.445906944e11 - t189 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t192 * t207);
        let t212 = t27 * t211;
        let t213 = t212 * t151;
        let t216 = piecewise3(t123, 0.0, -3.0 / 8.0 * t132 * t213);
        let tzk0 = t122 + t216;
        zk[ip] += tzk0;
    }
}
