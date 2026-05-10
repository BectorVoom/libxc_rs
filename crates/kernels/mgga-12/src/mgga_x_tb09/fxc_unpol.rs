//! MGGA_X_TB09 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 41 shared lines across all orders.
//! Delta: 86 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_tb09_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    param_alpha: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        // --- shared preamble (41 lines) ---
        let t2 = M_CBRTPI;
        let t3 = param_c * t2;
        let t4 = M_CBRT2;
        let t5 = t4 * t4;
        let t6 = pow_1_3(rho[ip]);
        let t7 = t6 * t6;
        let t9 = 1.0 / t7 / rho[ip];
        let t14 = rho[ip] * rho[ip];
        let t16 = 1.0 / t7 / t14;
        let t20 = f64::abs(lapl[ip] * t9 / 6.0 - 0.53333333333333333332e0 * tau[ip] * t9 + 0.66666666666666666668e-1 * sigma[ip] * t16);
        let t22 = t5 * t20 < 0.5e-12;
        let t23 = lapl[ip] * t5;
        let t26 = tau[ip] * t5;
        let t27 = t26 * t9;
        let t29 = sigma[ip] * t5;
        let t32 = t23 * t9 / 6.0 - 0.53333333333333333333e0 * t27 + 0.66666666666666666667e-1 * t29 * t16;
        let t33 = 0.0 < t32;
        let t34 = piecewise3(t33, 0.5e-12, -0.5e-12);
        let t35 = piecewise3(t22, t34, t32);
        let t36 = xc_mgga_x_br89_get_x(t35);
        let t38 = f64::exp(t36 / 3.0);
        let t39 = f64::exp(-t36);
        let t41 = 1.0 + t36 / 2.0;
        let t42 = t39 * t41;
        let t43 = 1.0 - t42;
        let t44 = t38 * t43;
        let t45 = 1.0 / t36;
        let t46 = t44 * t45;
        let t51 = f64::sqrt(15.0);
        let t52 = (3.0 * param_c - 2.0) * t51;
        let t53 = 1.0 / M_PI;
        let t54 = M_SQRT2;
        let t55 = t53 * t54;
        let t56 = param_alpha * sigma[ip];
        let t57 = t5 * t16;
        let t60 = t27 - t56 * t57 / 8.0;
        let t61 = 0.1e-9 < t60;
        let t62 = piecewise3(t61, t60, 0.1e-9);
        let t63 = f64::sqrt(t62);
        let t68 = (-2.0 * t3 * t46 + t52 * t55 * t63 / 6.0) * t5;
        let tvrho0 = t68 * t6 / 2.0;
        vrho[ip] += tvrho0;
        // --- fxc delta (this level) (86 lines) ---
        let t70 = param_c * M_PI;
        let t71 = piecewise3(t33, 0.0, 0.0);
        let t74 = t26 * t16;
        let t78 = 1.0 / t7 / t14 / rho[ip];
        let t82 = piecewise3(t22, t71, -5.0 / 18.0 * t23 * t16 + 0.88888888888888888889e0 * t74 - 0.17777777777777777778e0 * t29 * t78);
        let t83 = t35 * t35;
        let t84 = 1.0 / t83;
        let t85 = t82 * t84;
        let t87 = f64::exp(-2.0 / 3.0 * t36);
        let t88 = 1.0 / t87;
        let t89 = t85 * t88;
        let t90 = t70 * t89;
        let t91 = t36 * t36;
        let t93 = t91 - 2.0 * t36 + 3.0;
        let t94 = 1.0 / t93;
        let t95 = t36 - 2.0;
        let t96 = t95 * t95;
        let t97 = t94 * t96;
        let t98 = t97 * t46;
        let t101 = t2 * t2;
        let t102 = t101 * t82;
        let t103 = t84 * t88;
        let t104 = t102 * t103;
        let t105 = t97 * t42;
        let t107 = t102 * t84;
        let t108 = t88 * t94;
        let t109 = t96 * t39;
        let t110 = t108 * t109;
        let t113 = t104 * t105 - t107 * t110 / 2.0;
        let t114 = t38 * t113;
        let t115 = t114 * t45;
        let t118 = 1.0 / t91;
        let t119 = t44 * t118;
        let t120 = t70 * t119;
        let t121 = t108 * t96;
        let t122 = t85 * t121;
        let t125 = t52 * t53;
        let t127 = t54 / t63;
        let t129 = t5 * t78;
        let t133 = piecewise3(t61, -5.0 / 3.0 * t74 + t56 * t129 / 3.0, 0.0);
        let t138 = (-2.0 / 3.0 * t90 * t98 - 2.0 * t3 * t115 + 2.0 * t120 * t122 + t125 * t127 * t133 / 12.0) * t5;
        let t141 = 1.0 / t7;
        let tv2rho20 = t138 * t6 / 2.0 + t68 * t141 / 6.0;
        v2rho2[ip] += tv2rho20;
        let t145 = piecewise3(t22, t71, 0.66666666666666666667e-1 * t57);
        let t146 = t145 * t84;
        let t147 = t146 * t88;
        let t148 = t70 * t147;
        let t151 = t101 * t145;
        let t152 = t151 * t103;
        let t154 = t151 * t84;
        let t157 = t152 * t105 - t154 * t110 / 2.0;
        let t158 = t38 * t157;
        let t159 = t158 * t45;
        let t162 = t146 * t121;
        let t165 = param_alpha * t5;
        let t168 = piecewise3(t61, -t165 * t16 / 8.0, 0.0);
        let t173 = (-2.0 / 3.0 * t148 * t98 - 2.0 * t3 * t159 + 2.0 * t120 * t162 + t125 * t127 * t168 / 12.0) * t5;
        let tv2rhosigma0 = t173 * t6 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t175 = t5 * t9;
        let t177 = piecewise3(t22, t71, t175 / 6.0);
        let t178 = t177 * t84;
        let t179 = t178 * t88;
        let t180 = t70 * t179;
        let t183 = t101 * t177;
        let t184 = t183 * t103;
        let t186 = t183 * t84;
        let t189 = t184 * t105 - t186 * t110 / 2.0;
        let t190 = t38 * t189;
        let t191 = t190 * t45;
        let t194 = t178 * t121;
        let t198 = (-2.0 / 3.0 * t180 * t98 - 2.0 * t3 * t191 + 2.0 * t120 * t194) * t5;
        let tv2rholapl0 = t198 * t6 / 2.0;
        v2rholapl[ip] += tv2rholapl0;
        let t201 = piecewise3(t22, t71, -0.53333333333333333333e0 * t175);
        let t202 = t201 * t84;
        let t203 = t202 * t88;
        let t204 = t70 * t203;
        let t207 = t101 * t201;
        let t208 = t207 * t103;
        let t210 = t207 * t84;
        let t213 = t208 * t105 - t210 * t110 / 2.0;
        let t214 = t38 * t213;
        let t215 = t214 * t45;
        let t218 = t202 * t121;
        let t221 = piecewise3(t61, t175, 0.0);
        let t226 = (-2.0 / 3.0 * t204 * t98 - 2.0 * t3 * t215 + 2.0 * t120 * t218 + t125 * t127 * t221 / 12.0) * t5;
        let tv2rhotau0 = t226 * t6 / 2.0;
        v2rhotau[ip] += tv2rhotau0;
    }
}
