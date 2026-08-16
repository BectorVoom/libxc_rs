//! LDA_C_VWN_4 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_4.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_VWN_4 fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_vwn_4_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3::<f64>(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3::<f64>(rho[ip]);
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
        let t32 = M_PI * M_PI;
        let t33 = 1.0 / t32;
        let t35 = t11 + 0.534175 * t12 + 11.4813;
        let t36 = 1.0 / t35;
        let t40 = f64::ln(t4 * t9 * t36 / 4.0);
        let t41 = t12 + 1.06835;
        let t44 = f64::atan(6.692072046645942 / t41);
        let t46 = t26 + 0.228344;
        let t47 = t46 * t46;
        let t49 = f64::ln(t47 * t36);
        let t54 = pow_1_3::<f64>(zeta_threshold);
        let t56 = piecewise3::<f64>(1.0 <= zeta_threshold, t54 * zeta_threshold, 1.0);
        let t59 = M_CBRT2;
        let t60 = t59 - 1.0;
        let t65 = 9.0 * t56 - 9.0;
        let t67 = t33 * (t40 + 0.32323836906055065 * t44 + 0.021608710360898266 * t49) * t65 / 24.0;
        let tzk0 = t20 + t25 + t31 - t67;
        zk[ip] += tzk0;
        let t69 = 1.0 / t7 / rho[ip];
        let t70 = t6 * t69;
        let t74 = t4 * t6;
        let t75 = t14 * t14;
        let t76 = 1.0 / t75;
        let t77 = t8 * t76;
        let t78 = t4 * t70;
        let t79 = t78 / 12.0;
        let t80 = 1.0 / t12;
        let t81 = t80 * t1;
        let t82 = t3 * t6;
        let t84 = t81 * t82 * t69;
        let t86 = -t79 - 0.31062 * t84;
        let t91 = t1 * t1;
        let t93 = 1.0 / t3;
        let t94 = (-t4 * t70 * t15 / 12.0 - t74 * t77 * t86 / 4.0) * t91 * t93;
        let t95 = t5 * t7;
        let t96 = t95 * t14;
        let t97 = t94 * t96;
        let t99 = t21 * t21;
        let t100 = 1.0 / t99;
        let t102 = t100 * t80 * t1;
        let t104 = 37.8469910464 * t100 + 1.0;
        let t105 = 1.0 / t104;
        let t108 = t102 * t82 * t69 * t105;
        let t110 = t27 * t15;
        let t111 = t110 * t80;
        let t114 = t28 * t76;
        let t116 = -t111 * t78 / 6.0 - t114 * t86;
        let t117 = 1.0 / t28;
        let t118 = t116 * t117;
        let t119 = t118 * t14;
        let t124 = t35 * t35;
        let t125 = 1.0 / t124;
        let t126 = t8 * t125;
        let t128 = -t79 - 0.08902916666666667 * t84;
        let t134 = (-t4 * t70 * t36 / 12.0 - t74 * t126 * t128 / 4.0) * t91 * t93;
        let t135 = t95 * t35;
        let t138 = t41 * t41;
        let t139 = 1.0 / t138;
        let t141 = t139 * t80 * t1;
        let t143 = 44.7838282775 * t139 + 1.0;
        let t144 = 1.0 / t143;
        let t149 = t46 * t36;
        let t150 = t149 * t80;
        let t153 = t47 * t125;
        let t155 = -t150 * t78 / 6.0 - t153 * t128;
        let t156 = 1.0 / t47;
        let t157 = t155 * t156;
        let t162 = t33 * (t134 * t135 / 3.0 + 0.36052240899892257 * t141 * t82 * t69 * t144 + 0.021608710360898266 * t157 * t35) * t65;
        let tvrho0 = t20 + t25 + t31 - t67 + rho[ip] * (0.010363566666666667 * t97 + 0.03976574567502677 * t108 + 0.0009690227711544374 * t119 - t162 / 24.0);
        vrho[ip] += tvrho0;
        let t170 = rho[ip] * rho[ip];
        let t172 = 1.0 / t7 / t170;
        let t173 = t6 * t172;
        let t175 = t4 * t173 * t15;
        let t177 = t69 * t76;
        let t182 = 1.0 / t75 / t14;
        let t183 = t8 * t182;
        let t184 = t86 * t86;
        let t188 = t4 * t173;
        let t189 = t188 / 9.0;
        let t191 = 1.0 / t12 / t10;
        let t192 = t191 * t91;
        let t193 = t3 * t3;
        let t194 = t193 * t5;
        let t195 = t7 * t7;
        let t197 = 1.0 / t195 / t170;
        let t199 = t192 * t194 * t197;
        let t202 = t81 * t82 * t172;
        let t204 = t189 - 0.20708 * t199 + 0.41416 * t202;
        let t210 = (t175 / 9.0 + t74 * t177 * t86 / 6.0 + t74 * t183 * t184 / 2.0 - t74 * t77 * t204 / 4.0) * t91 * t93;
        let t211 = t210 * t96;
        let t214 = t5 / t195;
        let t215 = t214 * t14;
        let t216 = t94 * t215;
        let t218 = t95 * t86;
        let t219 = t94 * t218;
        let t221 = t99 * t21;
        let t222 = 1.0 / t221;
        let t223 = t222 * t1;
        let t224 = t223 * t3;
        let t226 = t224 * t173 * t105;
        let t229 = t100 * t191 * t91;
        let t232 = t229 * t194 * t197 * t105;
        let t236 = t102 * t82 * t172 * t105;
        let t238 = t99 * t99;
        let t240 = 1.0 / t238 / t21;
        let t241 = t240 * t1;
        let t242 = t241 * t3;
        let t243 = t104 * t104;
        let t244 = 1.0 / t243;
        let t246 = t242 * t173 * t244;
        let t249 = t27 * t76;
        let t250 = t249 * t81;
        let t251 = t69 * t86;
        let t255 = t110 * t191;
        let t256 = t91 * t193;
        let t257 = t5 * t197;
        let t258 = t256 * t257;
        let t263 = t28 * t182;
        let t267 = t175 / 72.0 + t250 * t82 * t251 / 3.0 - t255 * t258 / 9.0 + 2.0 / 9.0 * t111 * t188 + 2.0 * t263 * t184 - t114 * t204;
        let t268 = t267 * t117;
        let t269 = t268 * t14;
        let t272 = 1.0 / t28 / t27;
        let t273 = t116 * t272;
        let t274 = t14 * t80;
        let t275 = t273 * t274;
        let t276 = t275 * t78;
        let t278 = t118 * t86;
        let t281 = t4 * t173 * t36;
        let t283 = t69 * t125;
        let t288 = 1.0 / t124 / t35;
        let t289 = t8 * t288;
        let t290 = t128 * t128;
        let t296 = t189 - 0.059352777777777775 * t199 + 0.11870555555555555 * t202;
        let t302 = (t281 / 9.0 + t74 * t283 * t128 / 6.0 + t74 * t289 * t290 / 2.0 - t74 * t126 * t296 / 4.0) * t91 * t93;
        let t305 = t214 * t35;
        let t308 = t95 * t128;
        let t311 = t138 * t41;
        let t312 = 1.0 / t311;
        let t313 = t312 * t1;
        let t314 = t313 * t3;
        let t319 = t139 * t191 * t91;
        let t328 = t138 * t138;
        let t330 = 1.0 / t328 / t41;
        let t331 = t330 * t1;
        let t332 = t331 * t3;
        let t333 = t143 * t143;
        let t334 = 1.0 / t333;
        let t339 = t46 * t125;
        let t340 = t339 * t81;
        let t341 = t69 * t128;
        let t345 = t149 * t191;
        let t350 = t47 * t288;
        let t354 = t281 / 72.0 + t340 * t82 * t341 / 3.0 - t345 * t258 / 9.0 + 2.0 / 9.0 * t150 * t188 + 2.0 * t350 * t290 - t153 * t296;
        let t355 = t354 * t156;
        let t359 = 1.0 / t47 / t46;
        let t360 = t155 * t359;
        let t361 = t35 * t80;
        let t362 = t360 * t361;
        let t369 = t33 * (t302 * t135 / 3.0 + t134 * t305 / 9.0 + t134 * t308 / 3.0 + 0.12017413633297419 * t314 * t173 * t144 + 0.24034827266594838 * t319 * t194 * t197 * t144 - 0.48069654533189676 * t141 * t82 * t172 * t144 - 5.38185788493279 * t332 * t173 * t334 + 0.021608710360898266 * t355 * t35 + 0.003601451726816378 * t362 * t78 + 0.021608710360898266 * t157 * t128) * t65;
        let t371 = 0.010363566666666667 * t211 + 0.003454522222222222 * t216 + 0.010363566666666667 * t219 + 0.013255248558342257 * t226 + 0.026510497116684514 * t232 - 0.05302099423336903 * t236 - 0.5016712735053859 * t246 + 0.0009690227711544374 * t269 + 0.00016150379519240624 * t276 + 0.0009690227711544374 * t278 - t369 / 24.0;
        let tv2rho20 = 0.020727133333333335 * t97 + 0.07953149135005354 * t108 + 0.001938045542308875 * t119 - t162 / 12.0 + rho[ip] * t371;
        v2rho2[ip] += tv2rho20;
    }
}
