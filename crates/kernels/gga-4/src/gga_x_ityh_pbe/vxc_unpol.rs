//! GGA_X_ITYH_PBE vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 69 shared lines across all orders.
//! Delta: 65 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ityh_pbe_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_hyb_omega_0: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (69 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t6 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t21 = M_PI * t20;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = M_CBRT6;
        let t28 = param_mu * t27;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t34 = M_CBRT2;
        let t35 = t34 * t34;
        let t36 = sigma[ip] * t35;
        let t37 = rho[ip] * rho[ip];
        let t38 = t19 * t19;
        let t40 = 1.0 / t38 / t37;
        let t44 = param_kappa + t28 * t32 * t36 * t40 / 24.0;
        let t49 = 1.0 + param_kappa * (1.0 - param_kappa / t44);
        let t52 = t21 * t26 / t49;
        let t53 = f64::sqrt(t52);
        let t55 = param_hyb_omega_0 / t53;
        let t56 = t11 * rho[ip];
        let t57 = pow_1_3(t56);
        let t58 = 1.0 / t57;
        let t61 = t55 * t34 * t58 / 2.0;
        let t62 = 0.135e1 <= t61;
        let t63 = 0.135e1 < t61;
        let t64 = piecewise3(t63, t61, 0.135e1);
        let t65 = t64 * t64;
        let t68 = t65 * t65;
        let t69 = 1.0 / t68;
        let t71 = t68 * t65;
        let t72 = 1.0 / t71;
        let t74 = t68 * t68;
        let t75 = 1.0 / t74;
        let t78 = 1.0 / t74 / t65;
        let t81 = 1.0 / t74 / t68;
        let t84 = 1.0 / t74 / t71;
        let t86 = t74 * t74;
        let t87 = 1.0 / t86;
        let t90 = piecewise3(t63, 0.135e1, t61);
        let t91 = f64::sqrt(M_PI);
        let t92 = 1.0 / t90;
        let t94 = erf_approx(t92 / 2.0);
        let t96 = t90 * t90;
        let t97 = 1.0 / t96;
        let t99 = f64::exp(-t97 / 4.0);
        let t100 = t99 - 1.0;
        let t103 = t99 - 3.0 / 2.0 - 2.0 * t96 * t100;
        let t106 = 2.0 * t90 * t103 + t91 * t94;
        let t110 = piecewise3(t62, 1.0 / t65 / 36.0 - t69 / 960.0 + t72 / 26880.0 - t75 / 829440.0 + t78 / 28385280.0 - t81 / 0.107347968e10 + t84 / 0.445906944e11 - t87 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t90 * t106);
        let t115 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t110 * t49);
        let tzk0 = 2.0 * t115;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (65 lines) ---
        let t116 = 1.0 / t38;
        let t121 = t65 * t64;
        let t122 = 1.0 / t121;
        let t125 = param_hyb_omega_0 / t53 / t52;
        let t126 = t125 * t58;
        let t127 = t21 * t26;
        let t128 = t126 * t127;
        let t129 = t49 * t49;
        let t130 = 1.0 / t129;
        let t131 = param_kappa * param_kappa;
        let t132 = t130 * t131;
        let t133 = t44 * t44;
        let t134 = 1.0 / t133;
        let t135 = t134 * param_mu;
        let t136 = t132 * t135;
        let t137 = t27 * t32;
        let t138 = t37 * rho[ip];
        let t140 = 1.0 / t38 / t138;
        let t143 = t136 * t137 * sigma[ip] * t140;
        let t147 = 1.0 / t57 / t56;
        let t152 = -t128 * t143 / 18.0 - t55 * t34 * t147 * t11 / 6.0;
        let t153 = piecewise3(t63, t152, 0.0);
        let t156 = t68 * t64;
        let t157 = 1.0 / t156;
        let t160 = t68 * t121;
        let t161 = 1.0 / t160;
        let t165 = 1.0 / t74 / t64;
        let t169 = 1.0 / t74 / t121;
        let t173 = 1.0 / t74 / t156;
        let t177 = 1.0 / t74 / t160;
        let t181 = 1.0 / t86 / t64;
        let t185 = piecewise3(t63, 0.0, t152);
        let t187 = t99 * t97;
        let t191 = t96 * t90;
        let t192 = 1.0 / t191;
        let t196 = t90 * t100;
        let t201 = t192 * t185 * t99 / 2.0 - 4.0 * t196 * t185 - t92 * t185 * t99;
        let t204 = 2.0 * t185 * t103 - t187 * t185 + 2.0 * t90 * t201;
        let t208 = piecewise3(t62, -t122 * t153 / 18.0 + t157 * t153 / 240.0 - t161 * t153 / 4480.0 + t165 * t153 / 103680.0 - t169 * t153 / 2838528.0 + t173 * t153 / 89456640.0 - t177 * t153 / 0.31850496e10 + t181 * t153 / 0.1263403008e12, -8.0 / 3.0 * t185 * t106 - 8.0 / 3.0 * t90 * t204);
        let t214 = 1.0 / t19 / t138;
        let t219 = t32 * sigma[ip];
        let t220 = t219 * t35;
        let t221 = t135 * t27 * t220;
        let t225 = piecewise3(t2, 0.0, -t18 * t116 * t110 * t49 / 8.0 - 3.0 / 8.0 * t18 * t19 * t208 * t49 + t18 * t214 * t110 * t131 * t221 / 24.0);
        let tvrho0 = 2.0 * rho[ip] * t225 + 2.0 * t115;
        vrho[ip] += tvrho0;
        let t228 = t132 * t134;
        let t233 = t128 * t228 * t28 * t32 * t40 / 48.0;
        let t234 = piecewise3(t63, t233, 0.0);
        let t237 = t157 * t234;
        let t239 = t161 * t234;
        let t241 = t165 * t234;
        let t243 = t169 * t234;
        let t245 = t173 * t234;
        let t247 = t177 * t234;
        let t249 = t181 * t234;
        let t252 = piecewise3(t63, 0.0, t233);
        let t264 = t192 * t252 * t99 / 2.0 - 4.0 * t196 * t252 - t92 * t252 * t99;
        let t267 = 2.0 * t252 * t103 - t187 * t252 + 2.0 * t90 * t264;
        let t271 = piecewise3(t62, -t122 * t234 / 18.0 + t237 / 240.0 - t239 / 4480.0 + t241 / 103680.0 - t243 / 2838528.0 + t245 / 89456640.0 - t247 / 0.31850496e10 + t249 / 0.1263403008e12, -8.0 / 3.0 * t252 * t106 - 8.0 / 3.0 * t90 * t267);
        let t278 = t17 / t19 / t37;
        let t281 = t131 * t134;
        let t283 = t137 * t35;
        let t284 = t281 * param_mu * t283;
        let t288 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t271 * t49 - t6 * t278 * t110 * t284 / 64.0);
        let tvsigma0 = 2.0 * rho[ip] * t288;
        vsigma[ip] += tvsigma0;
    }
}
