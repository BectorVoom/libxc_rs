//! GGA_C_AM05 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_am05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(
    unused_imports,
    unused_variables,
    non_snake_case,
    clippy::excessive_precision,
    clippy::too_many_arguments,
    clippy::needless_return
)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::piecewise3;
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::rmath;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_am05_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_alpha: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.053425 * t10;
        let t13 = rmath::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 3.79785 * t13 + 0.8969 * t10 + 0.204775 * t16 + 0.123235 * t24;
        let t29 = 1.0 + 16.081979498692537 / t26;
        let t30 = rmath::ln(t29);
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.0278125 * t10;
        let t50 = 5.1785 * t13 + 0.905775 * t10 + 0.1100325 * t16 + 0.1241775 * t24;
        let t53 = 1.0 + 29.608749977793437 / t50;
        let t54 = rmath::ln(t53);
        let t58 = -0.0621814 * t12 * t30 + 0.0197516734986138 * t43 * t45 * t54;
        let t59 = piecewise3(t33, zeta_threshold, 1.0);
        let t60 = t58 * t59;
        let t61 = M_CBRT6;
        let t62 = param_alpha * t61;
        let t63 = M_PI * M_PI;
        let t64 = pow_1_3(t63);
        let t65 = t64 * t64;
        let t66 = 1.0 / t65;
        let t68 = t39 * t39;
        let t69 = sigma[ip] * t68;
        let t70 = rho[ip] * rho[ip];
        let t72 = 1.0 / t21 / t70;
        let t76 = 1.0 + t62 * t66 * t69 * t72 / 24.0;
        let t77 = 1.0 / t76;
        let t80 = t77 + param_gamma * (1.0 - t77);
        let tzk0 = t60 * t80;
        zk[ip] += tzk0;
        let t82 = 1.0 / t7 / rho[ip];
        let t83 = t6 * t82;
        let t87 = t26 * t26;
        let t88 = 1.0 / t87;
        let t89 = t12 * t88;
        let t91 = 1.0 / t13 * t1;
        let t92 = t3 * t6;
        let t93 = t92 * t82;
        let t94 = t91 * t93;
        let t96 = t4 * t83;
        let t98 = rmath::sqrt(t10);
        let t99 = t98 * t1;
        let t100 = t99 * t93;
        let t105 = t20 * t5 / t21 / rho[ip];
        let t107 = -0.632975 * t94
            - 0.29896666666666666 * t96
            - 0.1023875 * t100
            - 0.08215666666666667 * t105;
        let t108 = 1.0 / t29;
        let t109 = t107 * t108;
        let t112 = t43 * t1;
        let t117 = t43 * t45;
        let t118 = t50 * t50;
        let t119 = 1.0 / t118;
        let t124 = -0.8630833333333333 * t94 - 0.301925 * t96 - 0.05501625 * t100 - 0.082785 * t105;
        let t126 = 1.0 / t53;
        let t127 = t119 * t124 * t126;
        let t130 = 0.0011073470983333333 * t4 * t83 * t30 + 1.0 * t89 * t109
            - 0.00018311447306006544 * t112 * t92 * t82 * t54
            - 0.5848223622634646 * t117 * t127;
        let t131 = rho[ip] * t130;
        let t132 = t59 * t80;
        let t134 = rho[ip] * t58;
        let t135 = t76 * t76;
        let t136 = 1.0 / t135;
        let t138 = t136 * param_alpha * t61;
        let t139 = t66 * sigma[ip];
        let t140 = t70 * rho[ip];
        let t142 = 1.0 / t21 / t140;
        let t143 = t68 * t142;
        let t144 = t139 * t143;
        let t146 = param_gamma * t136;
        let t147 = t146 * t62;
        let t150 = t138 * t144 / 9.0 - t147 * t144 / 9.0;
        let t151 = t59 * t150;
        let tvrho0 = t131 * t132 + t134 * t151 + tzk0;
        vrho[ip] += tvrho0;
        let t153 = t66 * t68;
        let t156 = t146 * param_alpha;
        let t157 = t61 * t66;
        let t162 = t156 * t157 * t68 * t72 / 24.0 - t138 * t153 * t72 / 24.0;
        let t163 = t59 * t162;
        let tvsigma0 = t134 * t163;
        vsigma[ip] += tvsigma0;
        let t164 = t130 * t59;
        let t170 = 1.0 / t7 / t70;
        let t171 = t6 * t170;
        let t175 = t4 * t6;
        let t176 = t82 * t88;
        let t180 = t87 * t26;
        let t181 = 1.0 / t180;
        let t182 = t12 * t181;
        let t183 = t107 * t107;
        let t184 = t183 * t108;
        let t189 = 1.0 / t13 / t10 * t18;
        let t190 = t19 * t5;
        let t191 = t190 * t72;
        let t192 = t189 * t191;
        let t194 = t92 * t170;
        let t195 = t91 * t194;
        let t197 = t4 * t171;
        let t199 = 1.0 / rmath::sqrt(t10);
        let t200 = t199 * t18;
        let t201 = t200 * t191;
        let t203 = t99 * t194;
        let t206 = t20 * t5 * t72;
        let t208 = -0.4219833333333333 * t192
            + 0.8439666666666666 * t195
            + 0.3986222222222222 * t197
            + 0.06825833333333334 * t201
            + 0.13651666666666668 * t203
            + 0.1369277777777778 * t206;
        let t209 = t208 * t108;
        let t212 = t87 * t87;
        let t213 = 1.0 / t212;
        let t214 = t12 * t213;
        let t215 = t29 * t29;
        let t216 = 1.0 / t215;
        let t217 = t183 * t216;
        let t224 = t43 * t4;
        let t228 = t118 * t50;
        let t229 = 1.0 / t228;
        let t230 = t124 * t124;
        let t232 = t229 * t230 * t126;
        let t241 = -0.5753888888888888 * t192
            + 1.1507777777777777 * t195
            + 0.4025666666666667 * t197
            + 0.0366775 * t201
            + 0.073355 * t203
            + 0.137975 * t206;
        let t243 = t119 * t241 * t126;
        let t246 = t118 * t118;
        let t247 = 1.0 / t246;
        let t248 = t247 * t230;
        let t249 = t53 * t53;
        let t250 = 1.0 / t249;
        let t251 = t248 * t250;
        let t254 = -0.0014764627977777779 * t4 * t171 * t30
            - 0.035616666666666665 * t175 * t176 * t109
            - 2.0 * t182 * t184
            + 1.0 * t89 * t209
            + 16.081979498692537 * t214 * t217
            + 0.00024415263074675396 * t112 * t92 * t170 * t54
            + 0.01084358130030174 * t224 * t83 * t127
            + 1.1696447245269292 * t117 * t232
            - 0.5848223622634646 * t117 * t243
            - 17.315859105681465 * t117 * t251;
        let t255 = rho[ip] * t254;
        let t260 = 1.0 / t135 / t76;
        let t261 = param_alpha * param_alpha;
        let t263 = t61 * t61;
        let t264 = t260 * t261 * t263;
        let t266 = 1.0 / t64 / t63;
        let t267 = sigma[ip] * sigma[ip];
        let t268 = t266 * t267;
        let t269 = t70 * t70;
        let t272 = 1.0 / t7 / t269 / t140;
        let t273 = t39 * t272;
        let t274 = t268 * t273;
        let t278 = 1.0 / t21 / t269;
        let t279 = t68 * t278;
        let t280 = t139 * t279;
        let t283 = param_gamma * t260;
        let t285 = t283 * t261 * t263;
        let t290 = 4.0 / 81.0 * t264 * t274 - 11.0 / 27.0 * t138 * t280 - 4.0 / 81.0 * t285 * t274
            + 11.0 / 27.0 * t147 * t280;
        let t291 = t59 * t290;
        let tv2rho20 =
            2.0 * t131 * t151 + t255 * t132 + t134 * t291 + 2.0 * t60 * t150 + 2.0 * t164 * t80;
        v2rho2[ip] += tv2rho20;
        let t295 = t266 * t39;
        let t296 = t269 * t70;
        let t298 = 1.0 / t7 / t296;
        let t300 = t295 * t298 * sigma[ip];
        let t311 = -t264 * t300 / 54.0 + t138 * t153 * t142 / 9.0 + t285 * t300 / 54.0
            - t156 * t157 * t143 / 9.0;
        let t312 = t59 * t311;
        let tv2rhosigma0 = t131 * t163 + t134 * t312 + t60 * t162;
        v2rhosigma[ip] += tv2rhosigma0;
        let t314 = t269 * rho[ip];
        let t316 = 1.0 / t7 / t314;
        let t319 = t283 * t261;
        let t320 = t263 * t266;
        let t325 = -t319 * t320 * t39 * t316 / 144.0 + t264 * t295 * t316 / 144.0;
        let t326 = t59 * t325;
        let tv2sigma20 = t134 * t326;
        v2sigma2[ip] += tv2sigma20;
    }
}
