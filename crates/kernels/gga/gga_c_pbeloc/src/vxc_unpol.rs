//! GGA_C_PBELOC vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbeloc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_pbeloc_vxc_unpol(
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
        let t67 = rho[ip] * rho[ip];
        let t69 = 1.0 / t7 / t67;
        let t70 = sigma[ip] * t69;
        let t71 = 1.0 / t65;
        let t72 = t39 * t71;
        let t74 = 1.0 / t3;
        let t75 = t18 * t74;
        let t77 = f64::exp(-t24 / 4.0);
        let t78 = 1.0 - t77;
        let t79 = t5 * t78;
        let t80 = t75 * t79;
        let t83 = 0.375e-1 + 0.83333333333333333332e-3 * t70 * t72 * t80;
        let t85 = t71 * t18;
        let t87 = t85 * t74 * t5;
        let t90 = 1.0 / t59;
        let t91 = t83 * t90;
        let t94 = 1.0 / t66;
        let t97 = f64::exp(-(-t32 + t57) * t90 * t60 * t94);
        let t98 = t97 - 1.0;
        let t99 = 1.0 / t98;
        let t100 = t60 * t99;
        let t101 = sigma[ip] * sigma[ip];
        let t102 = t100 * t101;
        let t103 = t91 * t102;
        let t104 = t67 * t67;
        let t106 = 1.0 / t21 / t104;
        let t107 = t39 * t39;
        let t108 = t106 * t107;
        let t109 = t65 * t65;
        let t110 = 1.0 / t109;
        let t112 = 1.0 / t19;
        let t114 = t1 * t112 * t6;
        let t115 = t108 * t110 * t114;
        let t118 = t70 * t39 * t87 / 96.0 + t103 * t115 / 3072.0;
        let t119 = t83 * t118;
        let t120 = t90 * t60;
        let t121 = t100 * t118;
        let t123 = t121 * t91 + 1.0;
        let t124 = 1.0 / t123;
        let t125 = t120 * t124;
        let t127 = t119 * t125 + 1.0;
        let t128 = f64::ln(t127);
        let t130 = t62 * t66 * t128;
        let tzk0 = -t32 + t57 + t130;
        zk[ip] += tzk0;
        let t132 = 1.0 / t7 / rho[ip];
        let t133 = t6 * t132;
        let t135 = t4 * t133 * t30;
        let t136 = 0.11073470983333333333e-2 * t135;
        let t137 = t26 * t26;
        let t138 = 1.0 / t137;
        let t139 = t12 * t138;
        let t141 = 1.0 / t13 * t1;
        let t142 = t3 * t6;
        let t143 = t142 * t132;
        let t144 = t141 * t143;
        let t146 = t4 * t133;
        let t148 = f64::sqrt(t10);
        let t149 = t148 * t1;
        let t150 = t149 * t143;
        let t155 = t20 * t5 / t21 / rho[ip];
        let t157 = -0.632975e0 * t144 - 0.29896666666666666667e0 * t146 - 0.1023875e0 * t150 - 0.82156666666666666667e-1 * t155;
        let t158 = 1.0 / t29;
        let t159 = t157 * t158;
        let t160 = t139 * t159;
        let t161 = 1.0 * t160;
        let t162 = t43 * t1;
        let t165 = t162 * t142 * t132 * t54;
        let t166 = 0.18311447306006545054e-3 * t165;
        let t167 = t43 * t45;
        let t168 = t50 * t50;
        let t169 = 1.0 / t168;
        let t174 = -0.86308333333333333334e0 * t144 - 0.301925e0 * t146 - 0.5501625e-1 * t150 - 0.82785e-1 * t155;
        let t176 = 1.0 / t53;
        let t177 = t169 * t174 * t176;
        let t178 = t167 * t177;
        let t179 = 0.5848223622634646207e0 * t178;
        let t180 = t67 * rho[ip];
        let t182 = 1.0 / t7 / t180;
        let t183 = sigma[ip] * t182;
        let t187 = 1.0 / t104;
        let t190 = t6 * t77;
        let t191 = t4 * t190;
        let t194 = -0.19444444444444444444e-2 * t183 * t72 * t80 - 0.41666666666666666666e-3 * sigma[ip] * t187 * t72 * t191;
        let t195 = t194 * t118;
        let t200 = t194 * t90;
        let t201 = t200 * t102;
        let t204 = t59 * t59;
        let t205 = 1.0 / t204;
        let t206 = t83 * t205;
        let t207 = t60 * t60;
        let t208 = t206 * t207;
        let t209 = t98 * t98;
        let t210 = 1.0 / t209;
        let t211 = t210 * t101;
        let t212 = t211 * t106;
        let t213 = t208 * t212;
        let t214 = t109 * t66;
        let t215 = 1.0 / t214;
        let t217 = t107 * t215 * t1;
        let t218 = t112 * t6;
        let t219 = t136 + t161 - t166 - t179;
        let t220 = t219 * t97;
        let t222 = t217 * t218 * t220;
        let t225 = t104 * rho[ip];
        let t227 = 1.0 / t21 / t225;
        let t228 = t227 * t107;
        let t230 = t228 * t110 * t114;
        let t233 = -7.0 / 288.0 * t183 * t39 * t87 + t201 * t115 / 3072.0 + t213 * t222 / 3072.0 - 7.0 / 4608.0 * t103 * t230;
        let t234 = t83 * t233;
        let t236 = t119 * t90;
        let t237 = t123 * t123;
        let t238 = 1.0 / t237;
        let t239 = t60 * t238;
        let t241 = t207 * t210;
        let t242 = t206 * t241;
        let t243 = t118 * t219;
        let t244 = t94 * t97;
        let t245 = t243 * t244;
        let t247 = t100 * t233;
        let t249 = t121 * t200 + t242 * t245 + t247 * t91;
        let t250 = t239 * t249;
        let t252 = t125 * t195 + t125 * t234 - t236 * t250;
        let t254 = 1.0 / t127;
        let t256 = t62 * t66 * t252 * t254;
        let tvrho0 = -t32 + t57 + t130 + rho[ip] * (t136 + t161 - t166 - t179 + t256);
        vrho[ip] += tvrho0;
        let t259 = rho[ip] * t59;
        let t260 = t259 * t61;
        let t261 = t69 * t39;
        let t262 = t85 * t74;
        let t263 = t261 * t262;
        let t265 = t118 * t90 * t124;
        let t266 = t79 * t265;
        let t270 = t75 * t5;
        let t271 = t261 * t71 * t270;
        let t273 = t104 * t180;
        let t274 = 1.0 / t273;
        let t276 = 1.0 / t109 / t65;
        let t278 = t274 * t276 * t78;
        let t279 = t90 * t99;
        let t280 = t279 * t101;
        let t283 = t100 * sigma[ip];
        let t284 = t91 * t283;
        let t287 = t271 / 96.0 + 0.20186378047070195427e-3 * t278 * t280 + t284 * t115 / 1536.0;
        let t288 = t83 * t287;
        let t290 = t279 * t118;
        let t291 = t79 * t290;
        let t294 = t100 * t287;
        let t296 = 0.82246703342411321825e-2 * t263 * t291 + t91 * t294;
        let t297 = t239 * t296;
        let t299 = 0.82246703342411321825e-2 * t263 * t266 + t288 * t125 - t236 * t297;
        let t300 = t66 * t299;
        let t301 = t300 * t254;
        let tvsigma0 = t260 * t301;
        vsigma[ip] += tvsigma0;
    }
}
