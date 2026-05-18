//! HYB_LDA_XC_BN05 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/hyb_lda_xc_bn05.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

/// HYB_LDA_XC_BN05 vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_lda_xc_bn05_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3::<f64>(t2);
        let t4 = t3 * t1;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t6 * t4;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = 1.0 <= zeta_threshold;
        let t11 = pow_1_3::<f64>(zeta_threshold);
        let t13 = piecewise3::<f64>(t10, t11 * zeta_threshold, 1.0);
        let t14 = t13 * t9;
        let t15 = pow_1_3::<f64>(rho[ip]);
        let t16 = pow_1_3::<f64>(9.0);
        let t17 = t16 * t16;
        let t18 = t3 * t3;
        let t20 = param_hyb_omega_0 * t18 * t17;
        let t21 = 1.0 / t15;
        let t23 = piecewise3::<f64>(t10, t11, 1.0);
        let t24 = 1.0 / t23;
        let t27 = t24 * t21 * t1 * t20 / 18.0;
        let t28 = 1.92 <= t27;
        let t29 = 1.92 < t27;
        let t30 = piecewise3::<f64>(t29, t27, 1.92);
        let t31 = t30 * t30;
        let t34 = t31 * t31;
        let t35 = 1.0 / t34;
        let t37 = t34 * t31;
        let t38 = 1.0 / t37;
        let t40 = t34 * t34;
        let t41 = 1.0 / t40;
        let t43 = t40 * t31;
        let t44 = 1.0 / t43;
        let t46 = t40 * t34;
        let t47 = 1.0 / t46;
        let t49 = t40 * t37;
        let t50 = 1.0 / t49;
        let t52 = t40 * t40;
        let t53 = 1.0 / t52;
        let t56 = 1.0 / t52 / t31;
        let t59 = 1.0 / t52 / t34;
        let t62 = 1.0 / t52 / t37;
        let t65 = 1.0 / t52 / t40;
        let t68 = 1.0 / t52 / t43;
        let t71 = 1.0 / t52 / t46;
        let t74 = 1.0 / t52 / t49;
        let t76 = t52 * t52;
        let t77 = 1.0 / t76;
        let t80 = 1.0 / t76 / t31;
        let t83 = 1.0 / t76 / t34;
        let t85 = 1.0 / t31 / 9.0 - t35 / 30.0 + t38 / 70.0 - t41 / 135.0 + t44 / 231.0 - t47 / 364.0 + t50 / 540.0 - t53 / 765.0 + t56 / 1045.0 - t59 / 1386.0 + t62 / 1794.0 - t65 / 2275.0 + t68 / 2835.0 - t71 / 3480.0 + t74 / 4216.0 - t77 / 5049.0 + t80 / 5985.0 - t83 / 7030.0;
        let t86 = piecewise3::<f64>(t29, 1.92, t27);
        let t87 = f64::atan2(1.0, t86);
        let t88 = t86 * t86;
        let t89 = t88 + 3.0;
        let t90 = 1.0 / t88;
        let t91 = 1.0 + t90;
        let t92 = f64::ln(t91);
        let t94 = -t92 * t89 + 1.0;
        let t97 = t87 + t94 * t86 / 4.0;
        let t101 = piecewise3::<f64>(t28, t85, 1.0 - 8.0 / 3.0 * t97 * t86);
        let t105 = 3.0 / 16.0 * t101 * t15 * t14 * t7;
        let t107 = t21 * t6 * t4;
        let t109 = 1.0 + 0.053425 * t107;
        let t110 = f64::sqrt(t107);
        let t113 = pow_3_2::<f64>(t107);
        let t115 = t1 * t1;
        let t116 = t18 * t115;
        let t117 = t15 * t15;
        let t118 = 1.0 / t117;
        let t120 = t118 * t5 * t116;
        let t122 = 3.79785 * t110 + 0.8969 * t107 + 0.204775 * t113 + 0.123235 * t120;
        let t125 = 1.0 + 16.081979498692537 / t122;
        let t126 = f64::ln(t125);
        let t134 = 1.0 / (2.0 * t8 - 2.0) * (2.0 * t13 - 2.0);
        let t136 = 1.0 + 0.0278125 * t107;
        let t141 = 5.1785 * t110 + 0.905775 * t107 + 0.1100325 * t113 + 0.1241775 * t120;
        let t144 = 1.0 + 29.608749977793437 / t141;
        let t145 = f64::ln(t144);
        let t149 = -0.0621814 * t126 * t109 + 0.0197516734986138 * t145 * t136 * t134;
        let t152 = 3.2 - 0.225 * t107 + t120 / 4.0;
        let t153 = 1.0 / t152;
        let t155 = 3.4602 * t153 * t149;
        let tzk0 = -t105 + t155;
        zk[ip] += tzk0;
        let t158 = t101 * t118 * t14 * t7;
        let t160 = t31 * t30;
        let t161 = 1.0 / t160;
        let t163 = 1.0 / t15 / rho[ip];
        let t167 = t24 * t163 * t1 * t20 / 54.0;
        let t168 = piecewise3::<f64>(t29, -t167, 0.0);
        let t171 = t34 * t30;
        let t172 = 1.0 / t171;
        let t175 = t34 * t160;
        let t176 = 1.0 / t175;
        let t179 = t40 * t30;
        let t180 = 1.0 / t179;
        let t183 = t40 * t160;
        let t184 = 1.0 / t183;
        let t187 = t40 * t171;
        let t188 = 1.0 / t187;
        let t191 = t40 * t175;
        let t192 = 1.0 / t191;
        let t196 = 1.0 / t52 / t30;
        let t200 = 1.0 / t52 / t160;
        let t204 = 1.0 / t52 / t171;
        let t208 = 1.0 / t52 / t175;
        let t212 = 1.0 / t52 / t179;
        let t216 = 1.0 / t52 / t183;
        let t220 = 1.0 / t52 / t187;
        let t224 = 1.0 / t52 / t191;
        let t228 = 1.0 / t76 / t30;
        let t232 = 1.0 / t76 / t160;
        let t236 = 1.0 / t76 / t171;
        let t239 = -2.0 / 9.0 * t168 * t161 + 2.0 / 15.0 * t168 * t172 - 3.0 / 35.0 * t168 * t176 + 8.0 / 135.0 * t168 * t180 - 10.0 / 231.0 * t168 * t184 + 3.0 / 91.0 * t168 * t188 - 7.0 / 270.0 * t168 * t192 + 16.0 / 765.0 * t168 * t196 - 18.0 / 1045.0 * t168 * t200 + 10.0 / 693.0 * t168 * t204 - 11.0 / 897.0 * t168 * t208 + 24.0 / 2275.0 * t168 * t212 - 26.0 / 2835.0 * t168 * t216 + 7.0 / 870.0 * t168 * t220 - 15.0 / 2108.0 * t168 * t224 + 32.0 / 5049.0 * t168 * t228 - 34.0 / 5985.0 * t168 * t232 + 18.0 / 3515.0 * t168 * t236;
        let t240 = piecewise3::<f64>(t29, 0.0, -t167);
        let t243 = 1.0 / t91;
        let t249 = t88 * t86;
        let t250 = 1.0 / t249;
        let t251 = t250 * t89;
        let t252 = t243 * t240;
        let t255 = -2.0 * t92 * t240 * t86 + 2.0 * t252 * t251;
        let t258 = -t243 * t90 * t240 + t94 * t240 / 4.0 + t255 * t86 / 4.0;
        let t262 = piecewise3::<f64>(t28, t239, -8.0 / 3.0 * t97 * t240 - 8.0 / 3.0 * t258 * t86);
        let t265 = t262 * t15 * t14 * t7;
        let t267 = t163 * t6;
        let t271 = t122 * t122;
        let t272 = 1.0 / t271;
        let t273 = t272 * t109;
        let t275 = t1 / t110;
        let t276 = t6 * t3;
        let t277 = t163 * t276;
        let t278 = t277 * t275;
        let t280 = t267 * t4;
        let t282 = f64::sqrt(t107);
        let t283 = t1 * t282;
        let t284 = t277 * t283;
        let t287 = 1.0 / t117 / rho[ip];
        let t289 = t287 * t5 * t116;
        let t291 = -0.632975 * t278 - 0.29896666666666666 * t280 - 0.1023875 * t284 - 0.08215666666666667 * t289;
        let t292 = 1.0 / t125;
        let t293 = t292 * t291;
        let t296 = t1 * t134;
        let t301 = t136 * t134;
        let t302 = t141 * t141;
        let t303 = 1.0 / t302;
        let t308 = -0.8630833333333333 * t278 - 0.301925 * t280 - 0.05501625 * t284 - 0.082785 * t289;
        let t310 = 1.0 / t144;
        let t311 = t310 * t308 * t303;
        let t314 = 0.0011073470983333333 * t126 * t267 * t4 + 1.0 * t293 * t273 - 0.00018311447306006544 * t145 * t163 * t276 * t296 - 0.5848223622634646 * t311 * t301;
        let t315 = t153 * t314;
        let t317 = t152 * t152;
        let t318 = 1.0 / t317;
        let t319 = t318 * t149;
        let t322 = 0.075 * t280 - t289 / 6.0;
        let t323 = t322 * t319;
        let tvrho0 = -t105 + t155 + (-t158 / 16.0 - 3.0 / 16.0 * t265 + 3.4602 * t315 - 3.4602 * t323) * rho[ip];
        vrho[ip] += tvrho0;
    }
}
