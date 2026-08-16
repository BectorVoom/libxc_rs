//! GGA_C_OPTC exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_optc.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_optc_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
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
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t23 = t5 / t21;
        let t24 = t20 * t23;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081824322151104822e2 / t26;
        let t30 = f64::ln(t29);
        let t32 = 0.62182e-1 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t35 = t34 * zeta_threshold;
        let t36 = piecewise3(t33, t35, 1.0);
        let t39 = M_CBRT2;
        let t42 = 1.0 / (2.0 * t39 - 2.0);
        let t43 = (2.0 * t36 - 2.0) * t42;
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608574643216675549e2 / t50;
        let t54 = f64::ln(t53);
        let t57 = 0.19751789702565206229e-1 * t43 * t45 * t54;
        let t58 = M_PI * M_PI;
        let t59 = pow_1_3(t58);
        let t60 = t59 * t59;
        let t61 = t18 * t60;
        let t62 = t34 * t34;
        let t63 = piecewise3(t33, t62, 1.0);
        let t64 = t63 * t63;
        let t65 = t64 * t63;
        let t66 = 1.0 / t59;
        let t67 = t18 * t66;
        let t68 = rho[ip] * rho[ip];
        let t70 = 1.0 / t7 / t68;
        let t71 = sigma[ip] * t70;
        let t72 = t71 * t39;
        let t73 = 1.0 / t64;
        let t75 = 1.0 / t3;
        let t76 = t75 * t5;
        let t77 = t73 * t18 * t76;
        let t83 = 1.0 / t60;
        let t84 = t1 * t83;
        let t87 = f64::exp(-0.12897460341341234505e3 * (-t32 + t57) / t65 * t84);
        let t88 = t87 - 1.0;
        let t89 = 1.0 / t88;
        let t90 = t66 * t89;
        let t91 = sigma[ip] * sigma[ip];
        let t92 = t68 * t68;
        let t94 = 1.0 / t21 / t92;
        let t95 = t91 * t94;
        let t97 = t39 * t39;
        let t98 = t64 * t64;
        let t99 = 1.0 / t98;
        let t100 = t97 * t99;
        let t101 = 1.0 / t19;
        let t102 = t101 * t6;
        let t103 = t100 * t102;
        let t106 = t72 * t77 / 96.0 + 0.27166129655589868296e-2 * t90 * t95 * t103;
        let t107 = t1 * t66;
        let t109 = t107 * t89 * sigma[ip];
        let t110 = t70 * t39;
        let t112 = t73 * t75 * t5;
        let t116 = t18 * t83;
        let t117 = t88 * t88;
        let t118 = 1.0 / t117;
        let t119 = t118 * t91;
        let t120 = t116 * t119;
        let t121 = t94 * t97;
        let t122 = t99 * t101;
        let t123 = t122 * t6;
        let t124 = t121 * t123;
        let t127 = 1.0 + 0.86931614897887578546e-1 * t109 * t110 * t112 + 0.75571056687546295931e-2 * t120 * t124;
        let t128 = 1.0 / t127;
        let t132 = 1.0 + 0.27818116767324025134e1 * t67 * t106 * t128;
        let t133 = f64::ln(t132);
        let t137 = t2 * t59;
        let t140 = 0.2568e1 + 0.58165e1 * t10 + 0.184725e-2 * t24;
        let t143 = 1000.0 + 0.218075e4 * t10 + 118.0 * t24;
        let t144 = 1.0 / t143;
        let t146 = t140 * t144 - 0.18535714285714285714e-2;
        let t147 = t146 * t63;
        let t149 = t137 * t147 * sigma[ip];
        let t150 = t2 * t5;
        let t151 = pow_1_3(9.0);
        let t152 = t151 * t151;
        let t156 = 1.0 / t21 / t68;
        let t158 = sigma[ip] * t39;
        let t162 = f64::exp(-25.0 / 18.0 * t150 * t152 * t3 * t156 * t64 * t158);
        let t163 = t76 * t162;
        let t164 = t110 * t163;
        let t168 = param_c1 * (-t32 + t57 + 0.25844881434903430496e-2 * t61 * t65 * t133 + t149 * t164 / 2.0);
        let t169 = param_c2 - param_c1;
        let t171 = t4 * t9 * t39;
        let t173 = 1.0 + 0.53425e-1 * t171;
        let t174 = f64::sqrt(t171);
        let t177 = pow_3_2(t171);
        let t180 = t20 * t23 * t97;
        let t182 = 0.379785e1 * t174 + 0.8969e0 * t171 + 0.204775e0 * t177 + 0.123235e0 * t180;
        let t185 = 1.0 + 0.16081824322151104822e2 / t182;
        let t186 = f64::ln(t185);
        let t188 = 0.62182e-1 * t173 * t186;
        let t189 = 2.0 <= zeta_threshold;
        let t191 = piecewise3(t189, t35, 2.0 * t39);
        let t192 = 0.0 <= zeta_threshold;
        let t193 = piecewise3(t192, t35, 0.0);
        let t195 = (t191 + t193 - 2.0) * t42;
        let t197 = 1.0 + 0.5137e-1 * t171;
        let t202 = 0.705945e1 * t174 + 0.1549425e1 * t171 + 0.420775e0 * t177 + 0.1562925e0 * t180;
        let t205 = 1.0 + 0.32164683177870697974e2 / t202;
        let t206 = f64::ln(t205);
        let t210 = 1.0 + 0.278125e-1 * t171;
        let t215 = 0.51785e1 * t174 + 0.905775e0 * t171 + 0.1100325e0 * t177 + 0.1241775e0 * t180;
        let t218 = 1.0 + 0.29608574643216675549e2 / t215;
        let t219 = f64::ln(t218);
        let t220 = t210 * t219;
        let t223 = t195 * (-0.3109e-1 * t197 * t206 + t188 - 0.19751789702565206229e-1 * t220);
        let t225 = 0.19751789702565206229e-1 * t195 * t220;
        let t226 = piecewise3(t189, t62, t97);
        let t227 = piecewise3(t192, t62, 0.0);
        let t229 = t226 / 2.0 + t227 / 2.0;
        let t230 = t229 * t229;
        let t231 = t230 * t229;
        let t232 = 1.0 / t230;
        let t234 = t18 * t75;
        let t235 = t5 * t97;
        let t236 = t234 * t235;
        let t244 = f64::exp(-0.12897460341341234505e3 * (-t188 + t223 + t225) / t231 * t84);
        let t245 = t244 - 1.0;
        let t246 = 1.0 / t245;
        let t247 = t66 * t246;
        let t249 = t230 * t230;
        let t250 = 1.0 / t249;
        let t251 = t250 * t101;
        let t252 = t6 * t39;
        let t253 = t251 * t252;
        let t256 = t71 * t232 * t236 / 96.0 + 0.54332259311179736592e-2 * t247 * t95 * t253;
        let t258 = t107 * t246 * sigma[ip];
        let t259 = t70 * t232;
        let t260 = t76 * t97;
        let t264 = t245 * t245;
        let t265 = 1.0 / t264;
        let t266 = t265 * t91;
        let t267 = t116 * t266;
        let t268 = t94 * t250;
        let t269 = t102 * t39;
        let t270 = t268 * t269;
        let t273 = 1.0 + 0.86931614897887578543e-1 * t258 * t259 * t260 + 0.15114211337509259186e-1 * t267 * t270;
        let t274 = 1.0 / t273;
        let t278 = 1.0 + 0.27818116767324025134e1 * t67 * t256 * t274;
        let t279 = f64::ln(t278);
        let t285 = 0.2568e1 + 0.58165e1 * t171 + 0.184725e-2 * t180;
        let t288 = 1000.0 + 0.218075e4 * t171 + 118.0 * t180;
        let t289 = 1.0 / t288;
        let t291 = t285 * t289 - 0.18535714285714285714e-2;
        let t292 = t291 * t229;
        let t294 = t137 * t292 * sigma[ip];
        let t302 = f64::exp(-25.0 / 9.0 * t150 * t152 * t3 * t156 * t230 * sigma[ip]);
        let t303 = t235 * t302;
        let t304 = t70 * t75 * t303;
        let t309 = piecewise3(t33, zeta_threshold, 1.0);
        let t310 = t169 * (-t188 + t223 + t225 + 0.25844881434903430496e-2 * t61 * t231 * t279 + t294 * t304 / 2.0) * t309;
        let tzk0 = t168 + t310;
        zk[ip] += tzk0;
    }
}
