//! LDA_C_VWN_1 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_1_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
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
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = rmath::sqrt(t10);
        let t14 = t11 + 1.86372 * t12 + 12.9352;
        let t15 = 1.0 / t14;
        let t19 = rmath::ln(t4 * t9 * t15 / 4.0);
        let t21 = t12 + 3.72744;
        let t24 = rmath::atan(6.15199081975908 / t21);
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498;
        let t28 = t27 * t27;
        let t30 = rmath::ln(t28 * t15);
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(1.0 <= zeta_threshold, t34 * zeta_threshold, 1.0);
        let t38 = 2.0 * t36 - 2.0;
        let t39 = M_CBRT2;
        let t42 = 1.0 / (2.0 * t39 - 2.0);
        let t44 = -t38 * t42 + 1.0;
        let t45 = (0.0310907 * t19 + 0.038783294878113016 * t24 + 0.0009690227711544374 * t30) * t44;
        let t47 = t11 + 3.53021 * t12 + 18.0578;
        let t48 = 1.0 / t47;
        let t52 = rmath::ln(t4 * t9 * t48 / 4.0);
        let t54 = t12 + 7.06042;
        let t57 = rmath::atan(4.730926909560113 / t54);
        let t59 = t26 + 0.325;
        let t60 = t59 * t59;
        let t62 = rmath::ln(t60 * t48);
        let t66 = (0.01554535 * t52 + 0.05249139316978094 * t57 + 0.0022478670955426118 * t62) * t38 * t42;
        let tzk0 = t45 + t66;
        zk[ip] += tzk0;
        let t68 = 1.0 / t7 / rho[ip];
        let t69 = t6 * t68;
        let t73 = t4 * t6;
        let t74 = t14 * t14;
        let t75 = 1.0 / t74;
        let t76 = t8 * t75;
        let t77 = t4 * t69;
        let t78 = t77 / 12.0;
        let t79 = 1.0 / t12;
        let t80 = t79 * t1;
        let t81 = t3 * t6;
        let t83 = t80 * t81 * t68;
        let t85 = -t78 - 0.31062 * t83;
        let t90 = t1 * t1;
        let t92 = 1.0 / t3;
        let t93 = (-t4 * t69 * t15 / 12.0 - t73 * t76 * t85 / 4.0) * t90 * t92;
        let t94 = t5 * t7;
        let t95 = t94 * t14;
        let t98 = t21 * t21;
        let t99 = 1.0 / t98;
        let t101 = t99 * t79 * t1;
        let t103 = 37.8469910464 * t99 + 1.0;
        let t104 = 1.0 / t103;
        let t109 = t27 * t15;
        let t110 = t109 * t79;
        let t113 = t28 * t75;
        let t115 = -t110 * t77 / 6.0 - t113 * t85;
        let t116 = 1.0 / t28;
        let t117 = t115 * t116;
        let t121 = (0.010363566666666667 * t93 * t95 + 0.03976574567502677 * t101 * t81 * t68 * t104 + 0.0009690227711544374 * t117 * t14) * t44;
        let t125 = t47 * t47;
        let t126 = 1.0 / t125;
        let t127 = t8 * t126;
        let t129 = -t78 - 0.5883683333333334 * t83;
        let t135 = (-t4 * t69 * t48 / 12.0 - t73 * t127 * t129 / 4.0) * t90 * t92;
        let t136 = t94 * t47;
        let t139 = t54 * t54;
        let t140 = 1.0 / t139;
        let t142 = t140 * t79 * t1;
        let t144 = 22.3816694236 * t140 + 1.0;
        let t145 = 1.0 / t144;
        let t150 = t59 * t48;
        let t151 = t150 * t79;
        let t154 = t60 * t126;
        let t156 = -t151 * t77 / 6.0 - t154 * t129;
        let t157 = 1.0 / t60;
        let t158 = t156 * t157;
        let t163 = (0.005181783333333334 * t135 * t136 + 0.041388824077869424 * t142 * t81 * t68 * t145 + 0.0022478670955426118 * t158 * t47) * t38 * t42;
        let tvrho0 = t45 + t66 + rho[ip] * (t121 + t163);
        vrho[ip] += tvrho0;
        let t168 = rho[ip] * rho[ip];
        let t170 = 1.0 / t7 / t168;
        let t171 = t6 * t170;
        let t173 = t4 * t171 * t15;
        let t175 = t68 * t75;
        let t180 = 1.0 / t74 / t14;
        let t181 = t8 * t180;
        let t182 = t85 * t85;
        let t186 = t4 * t171;
        let t187 = t186 / 9.0;
        let t189 = 1.0 / t12 / t10;
        let t190 = t189 * t90;
        let t191 = t3 * t3;
        let t192 = t191 * t5;
        let t193 = t7 * t7;
        let t195 = 1.0 / t193 / t168;
        let t197 = t190 * t192 * t195;
        let t200 = t80 * t81 * t170;
        let t202 = t187 - 0.20708 * t197 + 0.41416 * t200;
        let t208 = (t173 / 9.0 + t73 * t175 * t85 / 6.0 + t73 * t181 * t182 / 2.0 - t73 * t76 * t202 / 4.0) * t90 * t92;
        let t212 = t5 / t193;
        let t213 = t212 * t14;
        let t216 = t94 * t85;
        let t219 = t98 * t21;
        let t221 = 1.0 / t219 * t1;
        let t222 = t221 * t3;
        let t227 = t99 * t189 * t90;
        let t236 = t98 * t98;
        let t238 = 1.0 / t236 / t21;
        let t239 = t238 * t1;
        let t240 = t239 * t3;
        let t241 = t103 * t103;
        let t242 = 1.0 / t241;
        let t247 = t27 * t75;
        let t248 = t247 * t80;
        let t249 = t68 * t85;
        let t253 = t109 * t189;
        let t254 = t90 * t191;
        let t255 = t5 * t195;
        let t256 = t254 * t255;
        let t261 = t28 * t180;
        let t265 = t173 / 72.0 + t248 * t81 * t249 / 3.0 - t253 * t256 / 9.0 + 2.0 / 9.0 * t110 * t186 + 2.0 * t261 * t182 - t113 * t202;
        let t266 = t265 * t116;
        let t270 = 1.0 / t28 / t27;
        let t271 = t115 * t270;
        let t272 = t14 * t79;
        let t273 = t271 * t272;
        let t279 = (0.010363566666666667 * t208 * t95 + 0.003454522222222222 * t93 * t213 + 0.010363566666666667 * t93 * t216 + 0.013255248558342257 * t222 * t171 * t104 + 0.026510497116684514 * t227 * t192 * t195 * t104 - 0.05302099423336903 * t101 * t81 * t170 * t104 - 0.5016712735053859 * t240 * t171 * t242 + 0.0009690227711544374 * t266 * t14 + 0.00016150379519240624 * t273 * t77 + 0.0009690227711544374 * t117 * t85) * t44;
        let t281 = t4 * t171 * t48;
        let t283 = t68 * t126;
        let t288 = 1.0 / t125 / t47;
        let t289 = t8 * t288;
        let t290 = t129 * t129;
        let t296 = t187 - 0.39224555555555557 * t197 + 0.7844911111111111 * t200;
        let t302 = (t281 / 9.0 + t73 * t283 * t129 / 6.0 + t73 * t289 * t290 / 2.0 - t73 * t127 * t296 / 4.0) * t90 * t92;
        let t305 = t212 * t47;
        let t308 = t94 * t129;
        let t311 = t139 * t54;
        let t313 = 1.0 / t311 * t1;
        let t314 = t313 * t3;
        let t319 = t140 * t189 * t90;
        let t328 = t139 * t139;
        let t330 = 1.0 / t328 / t54;
        let t331 = t330 * t1;
        let t332 = t331 * t3;
        let t333 = t144 * t144;
        let t334 = 1.0 / t333;
        let t339 = t59 * t126;
        let t340 = t339 * t80;
        let t341 = t68 * t129;
        let t345 = t150 * t189;
        let t350 = t60 * t288;
        let t354 = t281 / 72.0 + t340 * t81 * t341 / 3.0 - t345 * t256 / 9.0 + 2.0 / 9.0 * t151 * t186 + 2.0 * t350 * t290 - t154 * t296;
        let t355 = t354 * t157;
        let t359 = 1.0 / t60 / t59;
        let t360 = t156 * t359;
        let t361 = t47 * t79;
        let t362 = t360 * t361;
        let t369 = (0.005181783333333334 * t302 * t136 + 0.001727261111111111 * t135 * t305 + 0.005181783333333334 * t135 * t308 + 0.013796274692623142 * t314 * t171 * t145 + 0.027592549385246284 * t319 * t192 * t195 * t145 - 0.05518509877049257 * t142 * t81 * t170 * t145 - 0.3087836594474698 * t332 * t171 * t334 + 0.0022478670955426118 * t355 * t47 + 0.00037464451592376865 * t362 * t77 + 0.0022478670955426118 * t158 * t129) * t38 * t42;
        let tv2rho20 = 2.0 * t121 + 2.0 * t163 + rho[ip] * (t279 + t369);
        v2rho2[ip] += tv2rho20;
    }
}
