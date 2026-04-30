//! MGGA_K_CSK vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 86 shared lines across all orders.
//! Delta: 84 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_csk_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_csk_a: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (86 lines) ---
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t38 = t33 / t36;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t45 = t38 * sigma0 * t43;
        let t48 = 1.0 / t41 / rho0;
        let t53 = 5.0 / 54.0 * t38 * lapl0 * t48 - 5.0 / 81.0 * t45;
        let t55 = f64::ln(1.0 - f64::EPSILON);
        let t56 = 1.0 / param_csk_a;
        let t57 = f64::powf(-t55, -t56);
        let t58 = t53 < -t57;
        let t59 = f64::ln(f64::EPSILON);
        let t60 = f64::powf(-t59, -t56);
        let t61 = -t60 < t53;
        let t62 = piecewise3(t61, -t60, t53);
        let t63 = -t57 < t62;
        let t64 = piecewise3(t63, t62, -t57);
        let t65 = f64::abs(t64);
        let t66 = f64::powf(t65, param_csk_a);
        let t67 = 1.0 / t66;
        let t68 = f64::exp(-t67);
        let t69 = 1.0 - t68;
        let t70 = f64::powf(t69, t56);
        let t71 = piecewise5(t58, 0.0, t61, 1.0, t70);
        let t73 = 1.0 + 5.0 / 72.0 * t45 + t53 * t71;
        let t77 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t73);
        let t78 = rho1 <= dens_threshold;
        let t79 = -t18;
        let t81 = piecewise5(t16, t13, t12, t17, t79 * t9);
        let t82 = 1.0 + t81;
        let t83 = t82 <= zeta_threshold;
        let t84 = pow_1_3(t82);
        let t85 = t84 * t84;
        let t87 = piecewise3(t83, t25, t85 * t82);
        let t88 = t87 * t31;
        let t89 = rho1 * rho1;
        let t90 = pow_1_3(rho1);
        let t91 = t90 * t90;
        let t93 = 1.0 / t91 / t89;
        let t95 = t38 * sigma2 * t93;
        let t98 = 1.0 / t91 / rho1;
        let t103 = 5.0 / 54.0 * t38 * lapl1 * t98 - 5.0 / 81.0 * t95;
        let t104 = t103 < -t57;
        let t105 = -t60 < t103;
        let t106 = piecewise3(t105, -t60, t103);
        let t107 = -t57 < t106;
        let t108 = piecewise3(t107, t106, -t57);
        let t109 = f64::abs(t108);
        let t110 = f64::powf(t109, param_csk_a);
        let t111 = 1.0 / t110;
        let t112 = f64::exp(-t111);
        let t113 = 1.0 - t112;
        let t114 = f64::powf(t113, t56);
        let t115 = piecewise5(t104, 0.0, t105, 1.0, t114);
        let t117 = 1.0 + 5.0 / 72.0 * t95 + t103 * t115;
        let t121 = piecewise3(t78, 0.0, 3.0 / 20.0 * t7 * t88 * t117);
        let tzk0 = t77 + t121;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (84 lines) ---
        let t122 = t8 * t8;
        let t123 = 1.0 / t122;
        let t124 = t18 * t123;
        let t126 = piecewise5(t12, 0.0, t16, 0.0, t9 - t124);
        let t129 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t126);
        let t130 = t129 * t31;
        let t134 = 1.0 / t30;
        let t135 = t29 * t134;
        let t138 = t7 * t135 * t73 / 10.0;
        let t141 = 1.0 / t41 / t39 / rho0;
        let t143 = t38 * sigma0 * t141;
        let t149 = -25.0 / 162.0 * t38 * lapl0 * t43 + 40.0 / 243.0 * t143;
        let t151 = t70 * t67;
        let t152 = piecewise3(t61, 0.0, t149);
        let t153 = piecewise3(t63, t152, 0.0);
        let t155 = f64::abs(t64) / t64;
        let t156 = 1.0 / t65;
        let t158 = 1.0 / t69;
        let t159 = t68 * t158;
        let t160 = t155 * t156 * t159;
        let t162 = piecewise5(t58, 0.0, t61, 0.0, -t151 * t153 * t160);
        let t164 = -5.0 / 27.0 * t143 + t149 * t71 + t53 * t162;
        let t169 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t130 * t73 + t138 + 3.0 / 20.0 * t7 * t32 * t164);
        let t170 = t79 * t123;
        let t172 = piecewise5(t16, 0.0, t12, 0.0, -t9 - t170);
        let t175 = piecewise3(t83, 0.0, 5.0 / 3.0 * t85 * t172);
        let t176 = t175 * t31;
        let t180 = t87 * t134;
        let t183 = t7 * t180 * t117 / 10.0;
        let t185 = piecewise3(t78, 0.0, 3.0 / 20.0 * t7 * t176 * t117 + t183);
        let tvrho0 = t77 + t121 + t8 * (t169 + t185);
        vrho[ip * 2] += tvrho0;
        let t189 = piecewise5(t12, 0.0, t16, 0.0, -t9 - t124);
        let t192 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t189);
        let t193 = t192 * t31;
        let t198 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t193 * t73 + t138);
        let t200 = piecewise5(t16, 0.0, t12, 0.0, t9 - t170);
        let t203 = piecewise3(t83, 0.0, 5.0 / 3.0 * t85 * t200);
        let t204 = t203 * t31;
        let t210 = 1.0 / t91 / t89 / rho1;
        let t212 = t38 * sigma2 * t210;
        let t218 = -25.0 / 162.0 * t38 * lapl1 * t93 + 40.0 / 243.0 * t212;
        let t220 = t114 * t111;
        let t221 = piecewise3(t105, 0.0, t218);
        let t222 = piecewise3(t107, t221, 0.0);
        let t224 = f64::abs(t108) / t108;
        let t225 = 1.0 / t109;
        let t227 = 1.0 / t113;
        let t228 = t112 * t227;
        let t229 = t224 * t225 * t228;
        let t231 = piecewise5(t104, 0.0, t105, 0.0, -t220 * t222 * t229);
        let t233 = -5.0 / 27.0 * t212 + t218 * t115 + t103 * t231;
        let t238 = piecewise3(t78, 0.0, 3.0 / 20.0 * t7 * t204 * t117 + t183 + 3.0 / 20.0 * t7 * t88 * t233);
        let tvrho1 = t77 + t121 + t8 * (t198 + t238);
        vrho[ip * 2 + 1] += tvrho1;
        let t241 = t38 * t43;
        let t244 = t38 * t43 * t71;
        let t247 = piecewise3(t61, 0.0, -5.0 / 81.0 * t241);
        let t248 = piecewise3(t63, t247, 0.0);
        let t251 = piecewise5(t58, 0.0, t61, 0.0, -t151 * t248 * t160);
        let t253 = 5.0 / 72.0 * t241 - 5.0 / 81.0 * t244 + t53 * t251;
        let t257 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t253);
        let tvsigma0 = t8 * t257;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t258 = t38 * t93;
        let t261 = t38 * t93 * t115;
        let t264 = piecewise3(t105, 0.0, -5.0 / 81.0 * t258);
        let t265 = piecewise3(t107, t264, 0.0);
        let t268 = piecewise5(t104, 0.0, t105, 0.0, -t220 * t265 * t229);
        let t270 = 5.0 / 72.0 * t258 - 5.0 / 81.0 * t261 + t103 * t268;
        let t274 = piecewise3(t78, 0.0, 3.0 / 20.0 * t7 * t88 * t270);
        let tvsigma2 = t8 * t274;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t280 = piecewise3(t61, 0.0, 5.0 / 54.0 * t38 * t48);
        let t281 = piecewise3(t63, t280, 0.0);
        let t284 = piecewise5(t58, 0.0, t61, 0.0, -t151 * t281 * t160);
        let t286 = 5.0 / 54.0 * t38 * t48 * t71 + t53 * t284;
        let t290 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t286);
        let tvlapl0 = t8 * t290;
        vlapl[ip * 2] += tvlapl0;
        let t296 = piecewise3(t105, 0.0, 5.0 / 54.0 * t38 * t98);
        let t297 = piecewise3(t107, t296, 0.0);
        let t300 = piecewise5(t104, 0.0, t105, 0.0, -t220 * t297 * t229);
        let t302 = 5.0 / 54.0 * t38 * t98 * t115 + t103 * t300;
        let t306 = piecewise3(t78, 0.0, 3.0 / 20.0 * t7 * t88 * t302);
        let tvlapl1 = t8 * t306;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
