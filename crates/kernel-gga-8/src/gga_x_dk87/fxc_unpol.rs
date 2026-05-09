//! GGA_X_DK87 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 44 shared lines across all orders.
//! Delta: 48 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_dk87_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_a1: f64,
    param_alpha: f64,
    param_b1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (44 lines) ---
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
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = 1.0 / M_PI;
        let t21 = M_CBRT6;
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t26 = 1.0 / t25;
        let t27 = t3 * t3;
        let t29 = pow_1_3(t20);
        let t30 = 1.0 / t29;
        let t32 = t23 * t26 * t27 * t30;
        let t33 = M_CBRT4;
        let t34 = t33 * sigma[ip];
        let t35 = M_CBRT2;
        let t36 = t35 * t35;
        let t37 = t34 * t36;
        let t38 = rho[ip] * rho[ip];
        let t39 = t18 * t18;
        let t41 = 1.0 / t39 / t38;
        let t42 = f64::sqrt(sigma[ip]);
        let t47 = f64::powf(t42 * t35 / t18 / rho[ip], param_alpha);
        let t48 = param_a1 * t47;
        let t49 = 1.0 + t48;
        let t51 = param_b1 * sigma[ip];
        let t52 = t36 * t41;
        let t54 = t51 * t52 + 1.0;
        let t55 = 1.0 / t54;
        let t56 = t41 * t49 * t55;
        let t60 = 1.0 + 7.0 / 11664.0 * t32 * t37 * t56;
        let t64 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
        // --- vxc delta (29 lines) ---
        let t66 = t17 / t39;
        let t70 = t38 * rho[ip];
        let t72 = 1.0 / t39 / t70;
        let t74 = t72 * t49 * t55;
        let t78 = t23 * t26;
        let t79 = t27 * t30;
        let t81 = t78 * t79 * t33;
        let t82 = sigma[ip] * t36;
        let t85 = t48 * param_alpha * t55;
        let t89 = sigma[ip] * sigma[ip];
        let t90 = t89 * t35;
        let t91 = t38 * t38;
        let t92 = t91 * t38;
        let t94 = 1.0 / t18 / t92;
        let t96 = t54 * t54;
        let t97 = 1.0 / t96;
        let t99 = t49 * t97 * param_b1;
        let t103 = -7.0 / 4374.0 * t32 * t37 * t74 - 7.0 / 8748.0 * t81 * t82 * t72 * t85 + 7.0 / 2187.0 * t81 * t90 * t94 * t99;
        let t108 = piecewise3(t2, 0.0, -t6 * t66 * t60 / 8.0 - 3.0 / 8.0 * t6 * t19 * t103);
        let tvrho0 = 2.0 * rho[ip] * t108 + 2.0 * t64;
        vrho[ip] += tvrho0;
        let t111 = t33 * t36;
        let t115 = t52 * param_a1;
        let t116 = t47 * param_alpha;
        let t117 = t116 * t55;
        let t122 = t91 * rho[ip];
        let t124 = 1.0 / t18 / t122;
        let t129 = 7.0 / 11664.0 * t32 * t111 * t56 + 7.0 / 23328.0 * t81 * t115 * t117 - 7.0 / 5832.0 * t81 * sigma[ip] * t35 * t124 * t99;
        let t133 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t129);
        let tvsigma0 = 2.0 * rho[ip] * t133;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (48 lines) ---
        let t138 = t17 / t39 / rho[ip];
        let t146 = 1.0 / t39 / t91;
        let t148 = t146 * t49 * t55;
        let t152 = t82 * t146;
        let t156 = t91 * t70;
        let t158 = 1.0 / t18 / t156;
        let t163 = param_alpha * param_alpha;
        let t165 = t48 * t163 * t55;
        let t169 = t33 * t89;
        let t171 = t78 * t79 * t169;
        let t172 = t35 * t158;
        let t173 = t172 * param_a1;
        let t174 = t97 * param_b1;
        let t175 = t116 * t174;
        let t176 = t173 * t175;
        let t179 = t89 * sigma[ip];
        let t180 = t33 * t179;
        let t181 = t91 * t91;
        let t182 = t181 * t38;
        let t183 = 1.0 / t182;
        let t186 = 1.0 / t96 / t54;
        let t188 = param_b1 * param_b1;
        let t189 = t49 * t186 * t188;
        let t193 = 77.0 / 13122.0 * t32 * t37 * t148 + 133.0 / 26244.0 * t81 * t152 * t85 - 7.0 / 243.0 * t81 * t90 * t158 * t99 + 7.0 / 6561.0 * t81 * t152 * t165 - 56.0 / 6561.0 * t171 * t176 + 224.0 / 6561.0 * t32 * t180 * t183 * t189;
        let t198 = piecewise3(t2, 0.0, t6 * t138 * t60 / 12.0 - t6 * t66 * t103 / 4.0 - 3.0 / 8.0 * t6 * t19 * t193);
        let tv2rho20 = 2.0 * rho[ip] * t198 + 4.0 * t108;
        v2rho2[ip] += tv2rho20;
        let t208 = t36 * t72 * param_a1;
        let t212 = t35 * t94;
        let t214 = t174 * sigma[ip];
        let t218 = t47 * t163;
        let t219 = t218 * t55;
        let t223 = t33 * t35;
        let t225 = t78 * t79 * t223;
        let t227 = t94 * param_a1 * t47;
        let t229 = param_alpha * t97 * t51;
        let t233 = t181 * rho[ip];
        let t234 = 1.0 / t233;
        let t239 = -7.0 / 4374.0 * t32 * t111 * t74 - 7.0 / 4374.0 * t81 * t208 * t117 + 7.0 / 729.0 * t81 * t212 * t49 * t214 - 7.0 / 17496.0 * t81 * t208 * t219 + 7.0 / 2187.0 * t225 * t227 * t229 - 28.0 / 2187.0 * t32 * t169 * t234 * t189;
        let t244 = piecewise3(t2, 0.0, -t6 * t66 * t129 / 8.0 - 3.0 / 8.0 * t6 * t19 * t239);
        let tv2rhosigma0 = 2.0 * rho[ip] * t244 + 2.0 * t133;
        v2rhosigma[ip] += tv2rhosigma0;
        let t247 = 1.0 / sigma[ip];
        let t248 = t247 * t55;
        let t249 = t116 * t248;
        let t257 = t218 * t248;
        let t266 = 1.0 / t181;
        let t271 = 7.0 / 23328.0 * t81 * t115 * t249 - 7.0 / 2916.0 * t32 * t223 * t124 * t99 + 7.0 / 46656.0 * t81 * t115 * t257 - 7.0 / 5832.0 * t81 * t35 * t124 * param_a1 * t175 + 7.0 / 1458.0 * t32 * t34 * t266 * t189;
        let t275 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t271);
        let tv2sigma20 = 2.0 * rho[ip] * t275;
        v2sigma2[ip] += tv2sigma20;
    }
}
