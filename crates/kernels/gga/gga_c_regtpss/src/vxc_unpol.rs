//! GGA_C_REGTPSS vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_regtpss.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_regtpss_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081979498692535067e2 / t26;
        let t30 = f64::ln(t29);
        let t32 = 0.621814e-1 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608749977793437516e2 / t50;
        let t54 = f64::ln(t53);
        let t57 = 0.19751673498613801407e-1 * t43 * t45 * t54;
        let t58 = f64::ln(2.0);
        let t59 = 1.0 - t58;
        let t60 = M_PI * M_PI;
        let t61 = 1.0 / t60;
        let t62 = t59 * t61;
        let t63 = t34 * t34;
        let t64 = piecewise3(t33, t63, 1.0);
        let t65 = t64 * t64;
        let t66 = t65 * t64;
        let t68 = 1.0 + 0.25e-1 * t10;
        let t70 = 1.0 + 0.4445e-1 * t10;
        let t71 = 1.0 / t70;
        let t72 = t68 * t71;
        let t73 = rho[ip] * rho[ip];
        let t75 = 1.0 / t7 / t73;
        let t78 = 1.0 / t65;
        let t80 = 1.0 / t3;
        let t81 = t80 * t5;
        let t82 = t78 * t18 * t81;
        let t85 = 1.0 / t59;
        let t88 = 1.0 / t66;
        let t89 = t60 * t88;
        let t91 = f64::exp(-(-t32 + t57) * t85 * t89);
        let t92 = t91 - 1.0;
        let t93 = 1.0 / t92;
        let t94 = t85 * t93;
        let t95 = sigma[ip] * sigma[ip];
        let t96 = t94 * t95;
        let t97 = t72 * t96;
        let t98 = t73 * t73;
        let t100 = 1.0 / t21 / t98;
        let t101 = t39 * t39;
        let t102 = t100 * t101;
        let t103 = t65 * t65;
        let t104 = 1.0 / t103;
        let t105 = t102 * t104;
        let t106 = 1.0 / t19;
        let t107 = t1 * t106;
        let t108 = t107 * t6;
        let t109 = t105 * t108;
        let t112 = sigma[ip] * t75 * t39 * t82 / 96.0 + 0.21437009059034868486e-3 * t97 * t109;
        let t113 = t112 * t85;
        let t114 = t94 * t112;
        let t117 = 1.0 + 0.65854491829355115987e0 * t72 * t114;
        let t118 = 1.0 / t117;
        let t119 = t113 * t118;
        let t122 = 1.0 + 0.65854491829355115987e0 * t72 * t119;
        let t123 = f64::ln(t122);
        let t125 = t62 * t66 * t123;
        let tzk0 = -t32 + t57 + t125;
        zk[ip] += tzk0;
        let t127 = 1.0 / t7 / rho[ip];
        let t128 = t6 * t127;
        let t130 = t4 * t128 * t30;
        let t131 = 0.11073470983333333333e-2 * t130;
        let t132 = t26 * t26;
        let t133 = 1.0 / t132;
        let t134 = t12 * t133;
        let t136 = 1.0 / t13 * t1;
        let t137 = t3 * t6;
        let t138 = t137 * t127;
        let t139 = t136 * t138;
        let t141 = t4 * t128;
        let t143 = f64::sqrt(t10);
        let t144 = t143 * t1;
        let t145 = t144 * t138;
        let t150 = t20 * t5 / t21 / rho[ip];
        let t152 = -0.632975e0 * t139 - 0.29896666666666666667e0 * t141 - 0.1023875e0 * t145 - 0.82156666666666666667e-1 * t150;
        let t153 = 1.0 / t29;
        let t154 = t152 * t153;
        let t155 = t134 * t154;
        let t156 = 1.0 * t155;
        let t157 = t43 * t1;
        let t160 = t157 * t137 * t127 * t54;
        let t161 = 0.18311447306006545054e-3 * t160;
        let t162 = t43 * t45;
        let t163 = t50 * t50;
        let t164 = 1.0 / t163;
        let t169 = -0.86308333333333333334e0 * t139 - 0.301925e0 * t141 - 0.5501625e-1 * t145 - 0.82785e-1 * t150;
        let t171 = 1.0 / t53;
        let t172 = t164 * t169 * t171;
        let t173 = t162 * t172;
        let t174 = 0.5848223622634646207e0 * t173;
        let t175 = t71 * t112;
        let t176 = t85 * t118;
        let t177 = t175 * t176;
        let t180 = t70 * t70;
        let t181 = 1.0 / t180;
        let t182 = t68 * t181;
        let t183 = t182 * t113;
        let t184 = t118 * t1;
        let t185 = t184 * t138;
        let t188 = t73 * rho[ip];
        let t190 = 1.0 / t7 / t188;
        let t195 = t18 * t80;
        let t196 = t98 * t73;
        let t197 = 1.0 / t196;
        let t200 = t195 * t5 * t197 * t71;
        let t201 = t95 * t101;
        let t203 = t94 * t201 * t104;
        let t206 = t182 * t96;
        let t207 = t197 * t101;
        let t208 = t207 * t104;
        let t209 = t195 * t5;
        let t210 = t208 * t209;
        let t213 = t59 * t59;
        let t214 = 1.0 / t213;
        let t215 = t72 * t214;
        let t216 = t92 * t92;
        let t217 = 1.0 / t216;
        let t218 = t217 * t95;
        let t219 = t218 * t102;
        let t220 = t215 * t219;
        let t222 = 1.0 / t103 / t66;
        let t223 = t222 * t1;
        let t224 = t223 * t106;
        let t225 = t131 + t156 - t161 - t174;
        let t227 = t60 * t91;
        let t228 = t6 * t225 * t227;
        let t229 = t224 * t228;
        let t232 = t98 * rho[ip];
        let t234 = 1.0 / t21 / t232;
        let t235 = t234 * t101;
        let t236 = t235 * t104;
        let t237 = t236 * t108;
        let t240 = -7.0 / 288.0 * sigma[ip] * t190 * t39 * t82 - 0.71456696863449561619e-5 * t200 * t203 + 0.12705000702321332056e-4 * t206 * t210 + 0.21437009059034868486e-3 * t220 * t229 - 0.10003937560882938627e-2 * t97 * t237;
        let t241 = t240 * t85;
        let t242 = t241 * t118;
        let t245 = t72 * t112;
        let t246 = t117 * t117;
        let t247 = 1.0 / t246;
        let t248 = t85 * t247;
        let t249 = t71 * t85;
        let t250 = t93 * t112;
        let t251 = t249 * t250;
        let t254 = t182 * t94;
        let t255 = t112 * t1;
        let t259 = t214 * t217;
        let t260 = t72 * t259;
        let t261 = t112 * t225;
        let t262 = t89 * t91;
        let t263 = t261 * t262;
        let t266 = t94 * t240;
        let t269 = -0.54878743191129263322e-2 * t141 * t251 + 0.9757440539382783019e-2 * t254 * t255 * t138 + 0.65854491829355115987e0 * t260 * t263 + 0.65854491829355115987e0 * t72 * t266;
        let t270 = t248 * t269;
        let t273 = -0.54878743191129263322e-2 * t141 * t177 + 0.9757440539382783019e-2 * t183 * t185 + 0.65854491829355115987e0 * t72 * t242 - 0.65854491829355115987e0 * t245 * t270;
        let t275 = 1.0 / t122;
        let t277 = t62 * t66 * t273 * t275;
        let tvrho0 = -t32 + t57 + t125 + rho[ip] * (t131 + t156 - t161 - t174 + t277);
        vrho[ip] += tvrho0;
        let t280 = rho[ip] * t59;
        let t281 = t280 * t61;
        let t286 = t94 * sigma[ip];
        let t287 = t72 * t286;
        let t290 = t75 * t39 * t78 * t209 / 96.0 + 0.42874018118069736972e-3 * t287 * t109;
        let t291 = t290 * t85;
        let t292 = t291 * t118;
        let t295 = t68 * t68;
        let t296 = t295 * t181;
        let t297 = t296 * t112;
        let t298 = t214 * t247;
        let t299 = t93 * t290;
        let t300 = t298 * t299;
        let t303 = 0.65854491829355115987e0 * t72 * t292 - 0.4336814094102599731e0 * t297 * t300;
        let t304 = t66 * t303;
        let t305 = t304 * t275;
        let tvsigma0 = t281 * t305;
        vsigma[ip] += tvsigma0;
    }
}
