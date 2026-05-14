//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1098/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1098<F: Float>(t187: F, t6400: F, t1233: F, t15296: F, t1694: F, t18965: F, t18989: F, t18995: F, t18999: F, t19006: F, t19011: F, t19019: F, t19042: F, t19047: F, t19096: F, t19103: F, t20380: F, t20400: F, t20474: F, t20516: F, t3600: F, t4765: F, t5261: F, t6408: F, t6425: F, t6429: F, t972: F) -> (F,) {
    let t20524 = t187 * t6400;
    let t20549 = t18995 + t18999 + t187 * (t20380 + t20400 + t20474 + t20516) + 0.23392893589820816284e1 * t1233 * t19006 - 0.346315117987517266e2 * t5261 * t4765 - 0.58482233974552040708e0 * t20524 * t972 - 0.11696446794910408142e1 * t15296 * t1694 - 0.17315755899375863299e2 * t1233 * t18989 + 0.19751789702565206229e-1 * t187 * t18965 + 0.1038945353962551798e3 * t1233 * t19019 - t19042 - 0.17315755899375863299e2 * t3600 * t6429 - 0.58482233974552040708e0 * t1233 * t19096 - 0.35089340384731224426e1 * t1233 * t19011 + 0.11696446794910408142e1 * t3600 * t6408 - 0.58482233974552040708e0 * t3600 * t6425 - 0.1025389702100779493e4 * t1233 * t19103 - 0.34631511798751726598e2 * t1233 * t19047;
    (t20549,)
}
