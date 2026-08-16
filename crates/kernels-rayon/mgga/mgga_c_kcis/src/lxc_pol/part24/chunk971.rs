//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 971/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk971(t187: f64, t6400: f64, t1233: f64, t15296: f64, t1694: f64, t18965: f64, t18989: f64, t18995: f64, t18999: f64, t19006: f64, t19011: f64, t19019: f64, t19042: f64, t19047: f64, t19096: f64, t19103: f64, t20380: f64, t20400: f64, t20474: f64, t20516: f64, t3600: f64, t4765: f64, t5261: f64, t6408: f64, t6425: f64, t6429: f64, t972: f64) -> f64 {
    let t20524 = t187 * t6400;
    let t20549 = t18995 + t18999 + t187 * (t20380 + t20400 + t20474 + t20516) + 0.23392893589820816284e1_f64 * t1233 * t19006 - 0.346315117987517266e2_f64 * t5261 * t4765 - 0.58482233974552040708e0_f64 * t20524 * t972 - 0.11696446794910408142e1_f64 * t15296 * t1694 - 0.17315755899375863299e2_f64 * t1233 * t18989 + 0.19751789702565206229e-1_f64 * t187 * t18965 + 0.1038945353962551798e3_f64 * t1233 * t19019 - t19042 - 0.17315755899375863299e2_f64 * t3600 * t6429 - 0.58482233974552040708e0_f64 * t1233 * t19096 - 0.35089340384731224426e1_f64 * t1233 * t19011 + 0.11696446794910408142e1_f64 * t3600 * t6408 - 0.58482233974552040708e0_f64 * t3600 * t6425 - 0.1025389702100779493e4_f64 * t1233 * t19103 - 0.34631511798751726598e2_f64 * t1233 * t19047;
    t20549
}
