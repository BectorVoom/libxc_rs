//! GGA_X_LSPBE kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 32 shared lines across all orders.
//! Delta: 39 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_lspbe_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (32 lines) ---
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
        let t21 = param_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = param_kappa + t21 * t25 * t34 / 24.0;
        let t42 = param_kappa + 1.0;
        let t47 = f64::exp(-param_alpha * t20 * t25 * t34 / 24.0);
        let t50 = 1.0 + param_kappa * (1.0 - param_kappa / t37) - t42 * (1.0 - t47);
        let t54 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t50);
        let tzk0 = 2.0 * t54;
        zk[ip] += tzk0;
        // --- vxc delta (22 lines) ---
        let t56 = t17 / t31;
        let t60 = param_kappa * param_kappa;
        let t61 = t37 * t37;
        let t63 = t60 / t61;
        let t64 = t63 * t21;
        let t65 = t25 * sigma[ip];
        let t66 = t30 * rho[ip];
        let t68 = 1.0 / t31 / t66;
        let t69 = t28 * t68;
        let t72 = t42 * param_alpha;
        let t73 = t20 * t25;
        let t74 = t72 * t73;
        let t75 = t68 * t47;
        let t79 = t74 * t29 * t75 / 9.0 - t64 * t65 * t69 / 9.0;
        let t84 = piecewise3(t2, 0.0, -t6 * t56 * t50 / 8.0 - 3.0 / 8.0 * t6 * t19 * t79);
        let tvrho0 = 2.0 * rho[ip] * t84 + 2.0 * t54;
        vrho[ip] += tvrho0;
        let t87 = t63 * param_mu;
        let t91 = t72 * t20;
        let t92 = t25 * t28;
        let t97 = t87 * t73 * t28 * t33 / 24.0 - t91 * t92 * t33 * t47 / 24.0;
        let t101 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t97);
        let tvsigma0 = 2.0 * rho[ip] * t101;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (38 lines) ---
        let t106 = t17 / t31 / rho[ip];
        let t115 = t60 / t61 / t37;
        let t116 = param_mu * param_mu;
        let t117 = t20 * t20;
        let t119 = t115 * t116 * t117;
        let t121 = 1.0 / t23 / t22;
        let t122 = sigma[ip] * sigma[ip];
        let t123 = t121 * t122;
        let t124 = t30 * t30;
        let t127 = 1.0 / t18 / t124 / t66;
        let t128 = t27 * t127;
        let t133 = 1.0 / t31 / t124;
        let t134 = t28 * t133;
        let t138 = t133 * t47;
        let t142 = param_alpha * param_alpha;
        let t143 = t42 * t142;
        let t144 = t117 * t121;
        let t145 = t143 * t144;
        let t146 = t122 * t27;
        let t147 = t127 * t47;
        let t151 = -4.0 / 81.0 * t119 * t123 * t128 + 11.0 / 27.0 * t64 * t65 * t134 - 11.0 / 27.0 * t74 * t29 * t138 + 2.0 / 81.0 * t145 * t146 * t147;
        let t156 = piecewise3(t2, 0.0, t6 * t106 * t50 / 12.0 - t6 * t56 * t79 / 4.0 - 3.0 / 8.0 * t6 * t19 * t151);
        let tv2rho20 = 2.0 * rho[ip] * t156 + 4.0 * t84;
        v2rho2[ip] += tv2rho20;
        let t162 = t121 * t27;
        let t163 = t124 * t30;
        let t165 = 1.0 / t18 / t163;
        let t176 = t27 * t165;
        let t177 = sigma[ip] * t47;
        let t181 = t119 * t162 * t165 * sigma[ip] / 54.0 - t87 * t73 * t69 / 9.0 + t91 * t92 * t75 / 9.0 - t145 * t176 * t177 / 108.0;
        let t186 = piecewise3(t2, 0.0, -t6 * t56 * t97 / 8.0 - 3.0 / 8.0 * t6 * t19 * t181);
        let tv2rhosigma0 = 2.0 * rho[ip] * t186 + 2.0 * t101;
        v2rhosigma[ip] += tv2rhosigma0;
        let t189 = t115 * t116;
        let t190 = t124 * rho[ip];
        let t192 = 1.0 / t18 / t190;
        let t197 = t143 * t117;
        let t202 = -t189 * t144 * t27 * t192 / 144.0 + t197 * t162 * t192 * t47 / 288.0;
        let t206 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t202);
        let tv2sigma20 = 2.0 * rho[ip] * t206;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (this level) (39 lines) ---
        let t209 = t17 * t33;
        let t219 = t61 * t61;
        let t221 = t60 / t219;
        let t222 = t116 * param_mu;
        let t223 = t221 * t222;
        let t224 = t22 * t22;
        let t225 = 1.0 / t224;
        let t226 = t122 * sigma[ip];
        let t227 = t225 * t226;
        let t228 = t124 * t124;
        let t229 = t228 * t66;
        let t230 = 1.0 / t229;
        let t235 = 1.0 / t18 / t228;
        let t236 = t27 * t235;
        let t241 = 1.0 / t31 / t190;
        let t242 = t28 * t241;
        let t246 = t241 * t47;
        let t255 = t42 * t142 * param_alpha;
        let t256 = t255 * t225;
        let t261 = -16.0 / 81.0 * t223 * t227 * t230 + 44.0 / 81.0 * t119 * t123 * t236 - 154.0 / 81.0 * t64 * t65 * t242 + 154.0 / 81.0 * t74 * t29 * t246 - 22.0 / 81.0 * t145 * t146 * t235 * t47 + 8.0 / 243.0 * t256 * t226 * t230 * t47;
        let t266 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t209 * t50 + t6 * t106 * t79 / 4.0 - 3.0 / 8.0 * t6 * t56 * t151 - 3.0 / 8.0 * t6 * t19 * t261);
        let tv3rho30 = 2.0 * rho[ip] * t266 + 6.0 * t156;
        v3rho3[ip] += tv3rho30;
        let t276 = t228 * t30;
        let t277 = 1.0 / t276;
        let t278 = t225 * t277;
        let t299 = 2.0 / 27.0 * t223 * t278 * t122 - t119 * t162 * t127 * sigma[ip] / 6.0 + 11.0 / 27.0 * t87 * t73 * t134 - 11.0 / 27.0 * t91 * t92 * t138 + t145 * t128 * t177 / 12.0 - t256 * t277 * t122 * t47 / 81.0;
        let t304 = piecewise3(t2, 0.0, t6 * t106 * t97 / 12.0 - t6 * t56 * t181 / 4.0 - 3.0 / 8.0 * t6 * t19 * t299);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t304 + 4.0 * t186;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t310 = t228 * rho[ip];
        let t311 = 1.0 / t310;
        let t312 = t225 * t311;
        let t327 = -t223 * t312 * sigma[ip] / 36.0 + t189 * t144 * t176 / 27.0 - t197 * t162 * t165 * t47 / 54.0 + t256 * t311 * sigma[ip] * t47 / 216.0;
        let t332 = piecewise3(t2, 0.0, -t6 * t56 * t202 / 8.0 - 3.0 / 8.0 * t6 * t19 * t327);
        let tv3rhosigma20 = 2.0 * rho[ip] * t332 + 2.0 * t206;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t335 = t222 * t225;
        let t336 = 1.0 / t228;
        let t344 = t221 * t335 * t336 / 96.0 - t255 * t225 * t336 * t47 / 576.0;
        let t348 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t344);
        let tv3sigma30 = 2.0 * rho[ip] * t348;
        v3sigma3[ip] += tv3sigma30;
    }
}
