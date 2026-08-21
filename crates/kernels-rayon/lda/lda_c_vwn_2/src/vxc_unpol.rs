//! LDA_C_VWN_2 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_2_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let t20 = 0.0310907 * t19;
        let t21 = t12 + 3.72744;
        let t24 = rmath::atan(6.15199081975908 / t21);
        let t25 = 0.038783294878113016 * t24;
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498;
        let t28 = t27 * t27;
        let t30 = rmath::ln(t28 * t15);
        let t31 = 0.0009690227711544374 * t30;
        let t32 = M_PI * M_PI;
        let t33 = 1.0 / t32;
        let t35 = t11 + 0.534175 * t12 + 11.4813;
        let t36 = 1.0 / t35;
        let t40 = rmath::ln(t4 * t9 * t36 / 4.0);
        let t41 = t12 + 1.06835;
        let t44 = rmath::atan(6.692072046645942 / t41);
        let t46 = t26 + 0.228344;
        let t47 = t46 * t46;
        let t49 = rmath::ln(t47 * t36);
        let t54 = pow_1_3(zeta_threshold);
        let t56 = piecewise3(1.0 <= zeta_threshold, t54 * zeta_threshold, 1.0);
        let t58 = 2.0 * t56 - 2.0;
        let t59 = M_CBRT2;
        let t60 = t59 - 1.0;
        let t62 = 1.0 / t60 / 2.0;
        let t65 = 9.0 * t58 * t62 * t60;
        let t67 = t33 * (t40 + 0.32323836906055065 * t44 + 0.021608710360898266 * t49) * t65 / 24.0;
        let t69 = t11 + 10.06155 * t12 + 101.578;
        let t70 = 1.0 / t69;
        let t74 = rmath::ln(t4 * t9 * t70 / 4.0);
        let t76 = t12 + 20.1231;
        let t79 = rmath::atan(1.171685277708993 / t76);
        let t81 = t26 + 0.743294;
        let t82 = t81 * t81;
        let t84 = rmath::ln(t82 * t70);
        let t87 = t11 + 6.536 * t12 + 42.7198;
        let t88 = 1.0 / t87;
        let t92 = rmath::ln(t4 * t9 * t88 / 4.0);
        let t94 = t12 + 13.072;
        let t97 = rmath::atan(0.0448998886412873 / t94);
        let t99 = t26 + 0.409286;
        let t100 = t99 * t99;
        let t102 = rmath::ln(t100 * t88);
        let t106 = (0.01554535 * t74 + 0.6188180297906063 * t79 + 0.002667310007273315 * t84 - 0.0310907 * t92 - 20.521972937837504 * t97 - 0.004431373767749538 * t102) * t58 * t62;
        let t108 = t11 + 3.53021 * t12 + 18.0578;
        let t109 = 1.0 / t108;
        let t113 = rmath::ln(t4 * t9 * t109 / 4.0);
        let t115 = t12 + 7.06042;
        let t118 = rmath::atan(4.730926909560113 / t115);
        let t120 = t26 + 0.325;
        let t121 = t120 * t120;
        let t123 = rmath::ln(t121 * t109);
        let t127 = (0.01554535 * t113 + 0.05249139316978094 * t118 + 0.0022478670955426118 * t123 - t20 - t25 - t31) * t58 * t62;
        let tzk0 = t20 + t25 + t31 - t67 - t106 + t127;
        zk[ip] += tzk0;
        let t129 = 1.0 / t7 / rho[ip];
        let t130 = t6 * t129;
        let t134 = t4 * t6;
        let t135 = t14 * t14;
        let t136 = 1.0 / t135;
        let t137 = t8 * t136;
        let t138 = t4 * t130;
        let t139 = t138 / 12.0;
        let t140 = 1.0 / t12;
        let t141 = t140 * t1;
        let t142 = t3 * t6;
        let t144 = t141 * t142 * t129;
        let t146 = -t139 - 0.31062 * t144;
        let t151 = t1 * t1;
        let t153 = 1.0 / t3;
        let t154 = (-t4 * t130 * t15 / 12.0 - t134 * t137 * t146 / 4.0) * t151 * t153;
        let t155 = t5 * t7;
        let t156 = t155 * t14;
        let t157 = t154 * t156;
        let t158 = 0.010363566666666667 * t157;
        let t159 = t21 * t21;
        let t160 = 1.0 / t159;
        let t162 = t160 * t140 * t1;
        let t164 = 37.8469910464 * t160 + 1.0;
        let t165 = 1.0 / t164;
        let t168 = t162 * t142 * t129 * t165;
        let t169 = 0.03976574567502677 * t168;
        let t170 = t27 * t15;
        let t171 = t170 * t140;
        let t174 = t28 * t136;
        let t176 = -t171 * t138 / 6.0 - t174 * t146;
        let t177 = 1.0 / t28;
        let t178 = t176 * t177;
        let t179 = t178 * t14;
        let t180 = 0.0009690227711544374 * t179;
        let t184 = t35 * t35;
        let t185 = 1.0 / t184;
        let t186 = t8 * t185;
        let t188 = -t139 - 0.08902916666666667 * t144;
        let t194 = (-t4 * t130 * t36 / 12.0 - t134 * t186 * t188 / 4.0) * t151 * t153;
        let t195 = t155 * t35;
        let t198 = t41 * t41;
        let t199 = 1.0 / t198;
        let t201 = t199 * t140 * t1;
        let t203 = 44.7838282775 * t199 + 1.0;
        let t204 = 1.0 / t203;
        let t209 = t46 * t36;
        let t210 = t209 * t140;
        let t213 = t47 * t185;
        let t215 = -t210 * t138 / 6.0 - t213 * t188;
        let t216 = 1.0 / t47;
        let t217 = t215 * t216;
        let t222 = t33 * (t194 * t195 / 3.0 + 0.36052240899892257 * t201 * t142 * t129 * t204 + 0.021608710360898266 * t217 * t35) * t65;
        let t227 = t69 * t69;
        let t228 = 1.0 / t227;
        let t229 = t8 * t228;
        let t231 = -t139 - 1.676925 * t144;
        let t237 = (-t4 * t130 * t70 / 12.0 - t134 * t229 * t231 / 4.0) * t151 * t153;
        let t238 = t155 * t69;
        let t241 = t76 * t76;
        let t242 = 1.0 / t241;
        let t244 = t242 * t140 * t1;
        let t246 = 1.37284639 * t242 + 1.0;
        let t247 = 1.0 / t246;
        let t252 = t81 * t70;
        let t253 = t252 * t140;
        let t256 = t82 * t228;
        let t258 = -t253 * t138 / 6.0 - t256 * t231;
        let t259 = 1.0 / t82;
        let t260 = t258 * t259;
        let t266 = t87 * t87;
        let t267 = 1.0 / t266;
        let t268 = t8 * t267;
        let t270 = -t139 - 1.0893333333333333 * t144;
        let t276 = (-t4 * t130 * t88 / 12.0 - t134 * t268 * t270 / 4.0) * t151 * t153;
        let t277 = t155 * t87;
        let t280 = t94 * t94;
        let t281 = 1.0 / t280;
        let t283 = t281 * t140 * t1;
        let t285 = 0.002016 * t281 + 1.0;
        let t286 = 1.0 / t285;
        let t291 = t99 * t88;
        let t292 = t291 * t140;
        let t295 = t100 * t267;
        let t297 = -t292 * t138 / 6.0 - t295 * t270;
        let t298 = 1.0 / t100;
        let t299 = t297 * t298;
        let t304 = (0.005181783333333334 * t237 * t238 + 0.12084332918108974 * t244 * t142 * t129 * t247 + 0.002667310007273315 * t260 * t69 - 0.010363566666666667 * t276 * t277 - 0.15357238326806924 * t283 * t142 * t129 * t286 - 0.004431373767749538 * t299 * t87) * t58 * t62;
        let t308 = t108 * t108;
        let t309 = 1.0 / t308;
        let t310 = t8 * t309;
        let t312 = -t139 - 0.5883683333333334 * t144;
        let t318 = (-t4 * t130 * t109 / 12.0 - t134 * t310 * t312 / 4.0) * t151 * t153;
        let t319 = t155 * t108;
        let t322 = t115 * t115;
        let t323 = 1.0 / t322;
        let t325 = t323 * t140 * t1;
        let t327 = 22.3816694236 * t323 + 1.0;
        let t328 = 1.0 / t327;
        let t333 = t120 * t109;
        let t334 = t333 * t140;
        let t337 = t121 * t309;
        let t339 = -t334 * t138 / 6.0 - t337 * t312;
        let t340 = 1.0 / t121;
        let t341 = t339 * t340;
        let t346 = (0.005181783333333334 * t318 * t319 + 0.041388824077869424 * t325 * t142 * t129 * t328 + 0.0022478670955426118 * t341 * t108 - t158 - t169 - t180) * t58 * t62;
        let tvrho0 = t20 + t25 + t31 - t67 - t106 + t127 + rho[ip] * (t158 + t169 + t180 - t222 / 24.0 - t304 + t346);
        vrho[ip] += tvrho0;
    }
}
