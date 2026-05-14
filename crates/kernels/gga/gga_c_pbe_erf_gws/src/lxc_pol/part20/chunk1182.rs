//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1182/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1182<F: Float>(t13953: F, t15314: F, t11403: F, t3959: F, t1192: F, t12098: F, t14446: F, t15154: F, t20154: F, t2376: F, t2408: F, t2409: F, t3066: F, t3067: F, t335: F, t338: F, t353: F, t35889: F, t4007: F, t53807: F, t56199: F, t56836: F, t56840: F, t56843: F, t56847: F, t56849: F, t56853: F, t56882: F, t56907: F, t56928: F, t56951: F, t56977: F, t56996: F, t57025: F, t57056: F, t57081: F, t57106: F, t57125: F, t57148: F, t57170: F, t57193: F, t57212: F, t57237: F, t810: F, t8589: F, t8629: F, t8734: F, t898: F) -> (F,) {
    let t57260 = t13953 * t15314;
    let t57262 = t3959 * t11403;
    let t57264 = -t8629 * t20154 * t2376 * t56199 * t810 / 48.0 + t3066 * t2409 * t8734 * t15154 / 48.0 - 5.0 / 128.0 * t56836 - t56840 / 512.0 - t56843 / 48.0 - t56847 / 768.0 - t56849 / 96.0 + t56853 / 384.0 - t335 * t338 * t353 * t898 * (t56882 + t56907 + t56928 + t56951 + t56977 + t56996 + t57025 + t57056 + t57081 + t57106 + t57125 + t57148 + t57170 + t57193 + t57212 + t57237) / 96.0 - t53807 + t2408 * t2409 * t35889 * t4007 / 48.0 + t3066 * t2409 * t3067 * t1192 * t12098 / 48.0 + t2408 * t2409 * t8589 * t14446 / 24.0 + 7.0 / 288.0 * t57260 + t57262 / 24.0;
    (t57264,)
}
