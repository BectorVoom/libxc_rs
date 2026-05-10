//! GGA_K_LLP fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 40 shared lines across all orders.
//! Delta: 45 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_llp_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (40 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = param_beta * t4;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = M_CBRT4;
        let t29 = t27 * t28;
        let t30 = t24 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t22 / t34;
        let t37 = param_gamma * param_beta;
        let t38 = f64::sqrt(sigma[ip]);
        let t39 = t37 * t38;
        let t41 = 1.0 / t21 / rho[ip];
        let t45 = f64::ln(t38 * t31 * t41 + f64::sqrt(pow_2(t38 * t31 * t41) + 1.0));
        let t46 = t31 * t41 * t45;
        let t48 = 1.0 + t39 * t46;
        let t49 = 1.0 / t48;
        let t50 = t36 * t49;
        let t54 = 1.0 + 2.0 / 9.0 * t30 * t33 * t50;
        let t58 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        // --- vxc delta (28 lines) ---
        let t60 = t20 / t21;
        let t64 = t34 * rho[ip];
        let t66 = 1.0 / t22 / t64;
        let t67 = t66 * t49;
        let t71 = t48 * t48;
        let t72 = 1.0 / t71;
        let t73 = t36 * t72;
        let t75 = 1.0 / t21 / t34;
        let t77 = t31 * t75 * t45;
        let t79 = t37 * sigma[ip];
        let t80 = t32 * t66;
        let t82 = t33 * t36 + 1.0;
        let t83 = f64::sqrt(t82);
        let t84 = 1.0 / t83;
        let t85 = t80 * t84;
        let t88 = -4.0 / 3.0 * t39 * t77 - 4.0 / 3.0 * t79 * t85;
        let t93 = -16.0 / 27.0 * t30 * t33 * t67 - 2.0 / 9.0 * t30 * t33 * t73 * t88;
        let t98 = piecewise3(t2, 0.0, t7 * t60 * t54 / 10.0 + 3.0 / 20.0 * t7 * t23 * t93);
        let tvrho0 = 2.0 * rho[ip] * t98 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t101 = t24 * t27;
        let t102 = t28 * t32;
        let t106 = t37 / t38;
        let t108 = t32 * t36;
        let t109 = t108 * t84;
        let t112 = t106 * t46 / 2.0 + t37 * t109 / 2.0;
        let t117 = -2.0 / 9.0 * t30 * t33 * t73 * t112 + 2.0 / 9.0 * t101 * t102 * t50;
        let t121 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t117);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (45 lines) ---
        let t124 = t20 * t41;
        let t131 = t34 * t34;
        let t133 = 1.0 / t22 / t131;
        let t134 = t133 * t49;
        let t138 = t66 * t72;
        let t144 = 1.0 / t71 / t48;
        let t145 = t36 * t144;
        let t146 = t88 * t88;
        let t152 = 1.0 / t21 / t64;
        let t154 = t31 * t152 * t45;
        let t157 = t32 * t133;
        let t158 = t157 * t84;
        let t161 = sigma[ip] * sigma[ip];
        let t162 = t37 * t161;
        let t165 = 1.0 / t21 / t131 / t64;
        let t168 = 1.0 / t83 / t82;
        let t169 = t31 * t165 * t168;
        let t172 = 28.0 / 9.0 * t39 * t154 + 20.0 / 3.0 * t79 * t158 - 32.0 / 9.0 * t162 * t169;
        let t177 = 176.0 / 81.0 * t30 * t33 * t134 + 32.0 / 27.0 * t30 * t33 * t138 * t88 + 4.0 / 9.0 * t30 * t33 * t145 * t146 - 2.0 / 9.0 * t30 * t33 * t73 * t172;
        let t182 = piecewise3(t2, 0.0, -t7 * t124 * t54 / 30.0 + t7 * t60 * t93 / 5.0 + 3.0 / 20.0 * t7 * t23 * t177);
        let tv2rho20 = 2.0 * rho[ip] * t182 + 4.0 * t98;
        v2rho2[ip] += tv2rho20;
        let t191 = t72 * t88;
        let t200 = t24 * t29 * sigma[ip];
        let t201 = t144 * t112;
        let t202 = t201 * t88;
        let t203 = t108 * t202;
        let t210 = t37 * t31;
        let t211 = t131 * t34;
        let t213 = 1.0 / t21 / t211;
        let t218 = -2.0 / 3.0 * t106 * t77 - 2.0 * t37 * t85 + 4.0 / 3.0 * t210 * t213 * t168 * sigma[ip];
        let t223 = -16.0 / 27.0 * t101 * t102 * t67 - 2.0 / 9.0 * t30 * t108 * t191 + 16.0 / 27.0 * t30 * t33 * t138 * t112 + 4.0 / 9.0 * t200 * t203 - 2.0 / 9.0 * t30 * t33 * t73 * t218;
        let t228 = piecewise3(t2, 0.0, t7 * t60 * t117 / 10.0 + 3.0 / 20.0 * t7 * t23 * t223);
        let tv2rhosigma0 = 2.0 * rho[ip] * t228 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let t231 = t72 * t112;
        let t235 = t112 * t112;
        let t242 = t37 / t38 / sigma[ip];
        let t245 = 1.0 / sigma[ip];
        let t246 = t37 * t245;
        let t249 = t131 * rho[ip];
        let t252 = t31 / t21 / t249;
        let t253 = t252 * t168;
        let t256 = -t242 * t46 / 4.0 + t246 * t109 / 4.0 - t37 * t253 / 2.0;
        let t261 = -4.0 / 9.0 * t30 * t108 * t231 + 4.0 / 9.0 * t30 * t33 * t145 * t235 - 2.0 / 9.0 * t30 * t33 * t73 * t256;
        let t265 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t261);
        let tv2sigma20 = 2.0 * rho[ip] * t265;
        v2sigma2[ip] += tv2sigma20;
    }
}
