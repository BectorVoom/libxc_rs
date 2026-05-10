//! LDA_C_VWN_3 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 74 shared lines across all orders.
//! Delta: 120 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_VWN_3 vxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_vwn_3_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (74 lines) ---
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
        let t12 = f64::sqrt(t10);
        let t14 = t11 + 1.86372 * t12 + 12.9352;
        let t15 = 1.0 / t14;
        let t19 = f64::ln(t4 * t9 * t15 / 4.0);
        let t20 = 0.0310907 * t19;
        let t21 = t12 + 3.72744;
        let t24 = f64::atan(6.15199081975908 / t21);
        let t25 = 0.038783294878113016 * t24;
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498;
        let t28 = t27 * t27;
        let t30 = f64::ln(t28 * t15);
        let t31 = 0.0009690227711544374 * t30;
        let t33 = t11 + 3.53021 * t12 + 18.0578;
        let t34 = 1.0 / t33;
        let t38 = f64::ln(t4 * t9 * t34 / 4.0);
        let t40 = t12 + 7.06042;
        let t43 = f64::atan(4.730926909560113 / t40);
        let t45 = t26 + 0.325;
        let t46 = t45 * t45;
        let t48 = f64::ln(t46 * t34);
        let t50 = 0.01554535 * t38 + 0.05249139316978094 * t43 + 0.0022478670955426118 * t48 - t20 - t25 - t31;
        let t52 = t11 + 10.06155 * t12 + 101.578;
        let t53 = 1.0 / t52;
        let t57 = f64::ln(t4 * t9 * t53 / 4.0);
        let t59 = t12 + 20.1231;
        let t62 = f64::atan(1.171685277708993 / t59);
        let t64 = t26 + 0.743294;
        let t65 = t64 * t64;
        let t67 = f64::ln(t65 * t53);
        let t70 = t11 + 6.536 * t12 + 42.7198;
        let t71 = 1.0 / t70;
        let t75 = f64::ln(t4 * t9 * t71 / 4.0);
        let t77 = t12 + 13.072;
        let t80 = f64::atan(0.0448998886412873 / t77);
        let t82 = t26 + 0.409286;
        let t83 = t82 * t82;
        let t85 = f64::ln(t83 * t71);
        let t87 = 0.01554535 * t57 + 0.6188180297906063 * t62 + 0.002667310007273315 * t67 - 0.0310907 * t75 - 20.521972937837504 * t80 - 0.004431373767749538 * t85;
        let t88 = 1.0 / t87;
        let t90 = M_PI * M_PI;
        let t91 = 1.0 / t90;
        let t92 = t50 * t88 * t91;
        let t94 = t11 + 0.534175 * t12 + 11.4813;
        let t95 = 1.0 / t94;
        let t99 = f64::ln(t4 * t9 * t95 / 4.0);
        let t100 = t12 + 1.06835;
        let t103 = f64::atan(6.692072046645942 / t100);
        let t105 = t26 + 0.228344;
        let t106 = t105 * t105;
        let t108 = f64::ln(t106 * t95);
        let t110 = t99 + 0.32323836906055065 * t103 + 0.021608710360898266 * t108;
        let t112 = pow_1_3(zeta_threshold);
        let t114 = piecewise3(1.0 <= zeta_threshold, t112 * zeta_threshold, 1.0);
        let t116 = 2.0 * t114 - 2.0;
        let t118 = M_CBRT2;
        let t119 = t118 - 1.0;
        let t121 = 1.0 / t119 / 2.0;
        let t122 = 9.0 * t119;
        let t123 = t121 * t122;
        let t124 = t110 * t116 * t123;
        let t126 = t92 * t124 / 24.0;
        let tzk0 = t20 + t25 + t31 - t126;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (120 lines) ---
        let t128 = 1.0 / t7 / rho[ip];
        let t129 = t6 * t128;
        let t133 = t4 * t6;
        let t134 = t14 * t14;
        let t135 = 1.0 / t134;
        let t136 = t8 * t135;
        let t137 = t4 * t129;
        let t138 = t137 / 12.0;
        let t139 = 1.0 / t12;
        let t140 = t139 * t1;
        let t141 = t3 * t6;
        let t143 = t140 * t141 * t128;
        let t145 = -t138 - 0.31062 * t143;
        let t150 = t1 * t1;
        let t152 = 1.0 / t3;
        let t153 = (-t4 * t129 * t15 / 12.0 - t133 * t136 * t145 / 4.0) * t150 * t152;
        let t154 = t5 * t7;
        let t155 = t154 * t14;
        let t156 = t153 * t155;
        let t157 = 0.010363566666666667 * t156;
        let t158 = t21 * t21;
        let t159 = 1.0 / t158;
        let t161 = t159 * t139 * t1;
        let t163 = 37.8469910464 * t159 + 1.0;
        let t164 = 1.0 / t163;
        let t167 = t161 * t141 * t128 * t164;
        let t168 = 0.03976574567502677 * t167;
        let t169 = t27 * t15;
        let t170 = t169 * t139;
        let t173 = t28 * t135;
        let t175 = -t170 * t137 / 6.0 - t173 * t145;
        let t176 = 1.0 / t28;
        let t177 = t175 * t176;
        let t178 = t177 * t14;
        let t179 = 0.0009690227711544374 * t178;
        let t183 = t33 * t33;
        let t184 = 1.0 / t183;
        let t185 = t8 * t184;
        let t187 = -t138 - 0.5883683333333334 * t143;
        let t193 = (-t4 * t129 * t34 / 12.0 - t133 * t185 * t187 / 4.0) * t150 * t152;
        let t194 = t154 * t33;
        let t197 = t40 * t40;
        let t198 = 1.0 / t197;
        let t200 = t198 * t139 * t1;
        let t202 = 22.3816694236 * t198 + 1.0;
        let t203 = 1.0 / t202;
        let t208 = t45 * t34;
        let t209 = t208 * t139;
        let t212 = t46 * t184;
        let t214 = -t209 * t137 / 6.0 - t212 * t187;
        let t215 = 1.0 / t46;
        let t216 = t214 * t215;
        let t219 = 0.005181783333333334 * t193 * t194 + 0.041388824077869424 * t200 * t141 * t128 * t203 + 0.0022478670955426118 * t216 * t33 - t157 - t168 - t179;
        let t221 = t219 * t88 * t91;
        let t222 = t221 * t124;
        let t224 = t87 * t87;
        let t225 = 1.0 / t224;
        let t226 = t50 * t225;
        let t227 = t91 * t110;
        let t228 = t226 * t227;
        let t229 = t116 * t121;
        let t233 = t52 * t52;
        let t234 = 1.0 / t233;
        let t235 = t8 * t234;
        let t237 = -t138 - 1.676925 * t143;
        let t243 = (-t4 * t129 * t53 / 12.0 - t133 * t235 * t237 / 4.0) * t150 * t152;
        let t244 = t154 * t52;
        let t247 = t59 * t59;
        let t248 = 1.0 / t247;
        let t250 = t248 * t139 * t1;
        let t252 = 1.37284639 * t248 + 1.0;
        let t253 = 1.0 / t252;
        let t258 = t64 * t53;
        let t259 = t258 * t139;
        let t262 = t65 * t234;
        let t264 = -t259 * t137 / 6.0 - t262 * t237;
        let t265 = 1.0 / t65;
        let t266 = t264 * t265;
        let t272 = t70 * t70;
        let t273 = 1.0 / t272;
        let t274 = t8 * t273;
        let t276 = -t138 - 1.0893333333333333 * t143;
        let t282 = (-t4 * t129 * t71 / 12.0 - t133 * t274 * t276 / 4.0) * t150 * t152;
        let t283 = t154 * t70;
        let t286 = t77 * t77;
        let t287 = 1.0 / t286;
        let t289 = t287 * t139 * t1;
        let t291 = 0.002016 * t287 + 1.0;
        let t292 = 1.0 / t291;
        let t297 = t82 * t71;
        let t298 = t297 * t139;
        let t301 = t83 * t273;
        let t303 = -t298 * t137 / 6.0 - t301 * t276;
        let t304 = 1.0 / t83;
        let t305 = t303 * t304;
        let t308 = 0.005181783333333334 * t243 * t244 + 0.12084332918108974 * t250 * t141 * t128 * t253 + 0.002667310007273315 * t266 * t52 - 0.010363566666666667 * t282 * t283 - 0.15357238326806924 * t289 * t141 * t128 * t292 - 0.004431373767749538 * t305 * t70;
        let t309 = t122 * t308;
        let t310 = t229 * t309;
        let t311 = t228 * t310;
        let t316 = t94 * t94;
        let t317 = 1.0 / t316;
        let t318 = t8 * t317;
        let t320 = -t138 - 0.08902916666666667 * t143;
        let t326 = (-t4 * t129 * t95 / 12.0 - t133 * t318 * t320 / 4.0) * t150 * t152;
        let t327 = t154 * t94;
        let t330 = t100 * t100;
        let t331 = 1.0 / t330;
        let t333 = t331 * t139 * t1;
        let t335 = 44.7838282775 * t331 + 1.0;
        let t336 = 1.0 / t335;
        let t341 = t105 * t95;
        let t342 = t341 * t139;
        let t345 = t106 * t317;
        let t347 = -t342 * t137 / 6.0 - t345 * t320;
        let t348 = 1.0 / t106;
        let t349 = t347 * t348;
        let t352 = t326 * t327 / 3.0 + 0.36052240899892257 * t333 * t141 * t128 * t336 + 0.021608710360898266 * t349 * t94;
        let t354 = t352 * t116 * t123;
        let t355 = t92 * t354;
        let tvrho0 = t20 + t25 + t31 - t126 + rho[ip] * (t157 + t168 + t179 - t222 / 24.0 + t311 / 24.0 - t355 / 24.0);
        vrho[ip] += tvrho0;
    }
}
