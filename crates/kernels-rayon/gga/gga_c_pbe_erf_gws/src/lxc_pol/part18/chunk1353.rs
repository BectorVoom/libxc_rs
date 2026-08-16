//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1353/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1353(t13953: f64, t15314: f64, t11403: f64, t3959: f64, t1192: f64, t12098: f64, t14446: f64, t15154: f64, t20154: f64, t2376: f64, t2408: f64, t2409: f64, t3066: f64, t3067: f64, t335: f64, t338: f64, t353: f64, t35889: f64, t4007: f64, t53807: f64, t56199: f64, t56836: f64, t56840: f64, t56843: f64, t56847: f64, t56849: f64, t56853: f64, t56882: f64, t56907: f64, t56928: f64, t56951: f64, t56977: f64, t56996: f64, t57025: f64, t57056: f64, t57081: f64, t57106: f64, t57125: f64, t57148: f64, t57170: f64, t57193: f64, t57212: f64, t57237: f64, t810: f64, t8589: f64, t8629: f64, t8734: f64, t898: f64) -> f64 {
    let t57260 = t13953 * t15314;
    let t57262 = t3959 * t11403;
    let t57264 = -t8629 * t20154 * t2376 * t56199 * t810 / 48.0_f64 + t3066 * t2409 * t8734 * t15154 / 48.0_f64 - 5.0_f64 / 128.0_f64 * t56836 - t56840 / 512.0_f64 - t56843 / 48.0_f64 - t56847 / 768.0_f64 - t56849 / 96.0_f64 + t56853 / 384.0_f64 - t335 * t338 * t353 * t898 * (t56882 + t56907 + t56928 + t56951 + t56977 + t56996 + t57025 + t57056 + t57081 + t57106 + t57125 + t57148 + t57170 + t57193 + t57212 + t57237) / 96.0_f64 - t53807 + t2408 * t2409 * t35889 * t4007 / 48.0_f64 + t3066 * t2409 * t3067 * t1192 * t12098 / 48.0_f64 + t2408 * t2409 * t8589 * t14446 / 24.0_f64 + 7.0_f64 / 288.0_f64 * t57260 + t57262 / 24.0_f64;
    t57264
}
