//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 864/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk864<F: Float>(t15318: F, t15338: F, t3011: F, t98: F, t15283: F, t3015: F, t890: F, t101: F, t3014: F, t102: F, t15274: F, t15278: F, t15279: F, t15285: F, t15312: F, t15317: F, t15319: F, t15331: F, t15336: F, t3006: F, t3012: F, t69: F, t857: F, t863: F, t884: F, t889: F, t89: F, t918: F, t969: F) -> (F,) {
    let t15339 = t15318 * t15338;
    let t15343 = 1.0 / t3011 / t98;
    let t15345 = t15343 * t15283 * t3015;
    let t15349 = 1.0 / t3011 / t890;
    let t15352 = 1.0 / t3014 / t101;
    let t15353 = t15349 * t15283 * t15352;
    let t15363 = -0.51947267698127589897e2 * t889 * t3012 * t3006 * t15274 + 0.35089340384731224426e1 * t889 * t15278 * t15279 - 0.35089340384731224426e1 * t889 * t15285 - 0.58482233974552040708e0 * t889 * t15312 - 0.96490945932906628932e2 * t15317 * t15319 + 1.0 * t863 * t15331 + 0.51725014705706168417e3 * t15336 * t15339 + 0.1038945353962551798e3 * t889 * t15345 - 0.1025389702100779493e4 * t889 * t15353 + 0.34451131037037037036e-2 * t857 * t969 * t89 - 0.56969282336565386482e-3 * t884 * t69 * t918 * t102;
    (t15363,)
}
