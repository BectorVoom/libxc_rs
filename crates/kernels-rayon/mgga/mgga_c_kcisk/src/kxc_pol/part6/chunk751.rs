//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 751/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk751(t3011: f64, t890: f64, t101: f64, t3014: f64, t15283: f64, t102: f64, t15274: f64, t15278: f64, t15279: f64, t15285: f64, t15312: f64, t15317: f64, t15319: f64, t15331: f64, t15336: f64, t15339: f64, t15345: f64, t3006: f64, t3012: f64, t69: f64, t857: f64, t863: f64, t884: f64, t889: f64, t89: f64, t918: f64, t969: f64) -> f64 {
    let t15349 = 1.0_f64 / t3011 / t890;
    let t15352 = 1.0_f64 / t3014 / t101;
    let t15353 = t15349 * t15283 * t15352;
    let t15363 = -0.51947267698127589897e2_f64 * t889 * t3012 * t3006 * t15274 + 0.35089340384731224426e1_f64 * t889 * t15278 * t15279 - 0.35089340384731224426e1_f64 * t889 * t15285 - 0.58482233974552040708e0_f64 * t889 * t15312 - 0.96490945932906628932e2_f64 * t15317 * t15319 + 1.0_f64 * t863 * t15331 + 0.51725014705706168417e3_f64 * t15336 * t15339 + 0.1038945353962551798e3_f64 * t889 * t15345 - 0.1025389702100779493e4_f64 * t889 * t15353 + 0.34451131037037037036e-2_f64 * t857 * t969 * t89 - 0.56969282336565386482e-3_f64 * t884 * t69 * t918 * t102;
    t15363
}
