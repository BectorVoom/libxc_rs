//! GGA_X_SSB_SW fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 39 shared lines across all orders.
//! Delta: 41 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ssb_sw_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (39 lines) ---
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
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = param_B * t20 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t39 = 1.0 + param_C * t20 * t25 * t29 * t33 / 24.0;
        let t40 = 1.0 / t39;
        let t46 = param_D * t20 * t25;
        let t47 = t20 * t20;
        let t50 = 1.0 / t23 / t22;
        let t52 = sigma[ip] * sigma[ip];
        let t54 = t30 * t30;
        let t55 = t54 * rho[ip];
        let t57 = 1.0 / t18 / t55;
        let t61 = 1.0 + param_E * t47 * t50 * t52 * t27 * t57 / 288.0;
        let t62 = 1.0 / t61;
        let t67 = param_A + t26 * t29 * t33 * t40 / 24.0 - t46 * t29 * t33 * t62 / 24.0;
        let t71 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        // --- vxc delta (32 lines) ---
        let t73 = t17 / t31;
        let t77 = t30 * rho[ip];
        let t79 = 1.0 / t31 / t77;
        let t84 = param_B * t47;
        let t86 = t84 * t50 * t52;
        let t87 = t54 * t30;
        let t89 = 1.0 / t18 / t87;
        let t91 = t39 * t39;
        let t92 = 1.0 / t91;
        let t93 = t92 * param_C;
        let t94 = t27 * t89 * t93;
        let t101 = t22 * t22;
        let t102 = 1.0 / t101;
        let t103 = param_D * t102;
        let t104 = t52 * sigma[ip];
        let t105 = t103 * t104;
        let t106 = t54 * t54;
        let t107 = t106 * rho[ip];
        let t108 = 1.0 / t107;
        let t109 = t61 * t61;
        let t110 = 1.0 / t109;
        let t112 = t108 * t110 * param_E;
        let t115 = -t26 * t29 * t79 * t40 / 9.0 + t86 * t94 / 108.0 + t46 * t29 * t79 * t62 / 9.0 - t105 * t112 / 108.0;
        let t120 = piecewise3(t2, 0.0, -t6 * t73 * t67 / 8.0 - 3.0 / 8.0 * t6 * t19 * t115);
        let tvrho0 = 2.0 * rho[ip] * t120 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t123 = t28 * t33;
        let t130 = t27 * t57 * t93;
        let t137 = 1.0 / t106;
        let t139 = t137 * t110 * param_E;
        let t142 = t26 * t123 * t40 / 24.0 - t84 * t50 * sigma[ip] * t130 / 288.0 - t46 * t123 * t62 / 24.0 + t103 * t52 * t139 / 288.0;
        let t146 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t142);
        let tvsigma0 = 2.0 * rho[ip] * t146;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (41 lines) ---
        let t151 = t17 / t31 / rho[ip];
        let t159 = 1.0 / t31 / t54;
        let t164 = t54 * t77;
        let t166 = 1.0 / t18 / t164;
        let t168 = t27 * t166 * t93;
        let t171 = param_B * t102;
        let t172 = t171 * t104;
        let t173 = t106 * t30;
        let t174 = 1.0 / t173;
        let t176 = 1.0 / t91 / t39;
        let t178 = param_C * param_C;
        let t190 = t52 * t52;
        let t191 = t190 * sigma[ip];
        let t194 = 1.0 / t18 / t106 / t164;
        let t198 = 1.0 / t109 / t61;
        let t199 = param_E * param_E;
        let t202 = t47 * t50 * t27;
        let t203 = t198 * t199 * t202;
        let t206 = 11.0 / 27.0 * t26 * t29 * t159 * t40 - t86 * t168 / 12.0 + 2.0 / 81.0 * t172 * t174 * t176 * t178 - 11.0 / 27.0 * t46 * t29 * t159 * t62 + 35.0 / 324.0 * t105 * t174 * t110 * param_E - t103 * t191 * t194 * t203 / 2916.0;
        let t211 = piecewise3(t2, 0.0, t6 * t151 * t67 / 12.0 - t6 * t73 * t115 / 4.0 - 3.0 / 8.0 * t6 * t19 * t206);
        let tv2rho20 = 2.0 * rho[ip] * t211 + 4.0 * t120;
        v2rho2[ip] += tv2rho20;
        let t217 = t28 * t79;
        let t222 = t84 * t50 * t27;
        let t224 = param_C * sigma[ip];
        let t230 = t108 * t176 * t178;
        let t236 = t103 * t108;
        let t237 = t110 * param_E;
        let t238 = t237 * t52;
        let t241 = t106 * t87;
        let t243 = 1.0 / t18 / t241;
        let t248 = -t26 * t217 * t40 / 9.0 + t222 * t89 * t92 * t224 / 36.0 - t171 * t52 * t230 / 108.0 + t46 * t217 * t62 / 9.0 - t236 * t238 / 27.0 + t103 * t190 * t243 * t203 / 7776.0;
        let t253 = piecewise3(t2, 0.0, -t6 * t73 * t142 / 8.0 - 3.0 / 8.0 * t6 * t19 * t248);
        let tv2rhosigma0 = 2.0 * rho[ip] * t253 + 2.0 * t146;
        v2rhosigma[ip] += tv2rhosigma0;
        let t256 = t84 * t50;
        let t261 = t137 * t176 * t178;
        let t265 = t237 * sigma[ip];
        let t268 = t106 * t55;
        let t270 = 1.0 / t18 / t268;
        let t275 = -t256 * t130 / 144.0 + t171 * sigma[ip] * t261 / 288.0 + t103 * t137 * t265 / 96.0 - t103 * t104 * t270 * t203 / 20736.0;
        let t279 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t275);
        let tv2sigma20 = 2.0 * rho[ip] * t279;
        v2sigma2[ip] += tv2sigma20;
    }
}
