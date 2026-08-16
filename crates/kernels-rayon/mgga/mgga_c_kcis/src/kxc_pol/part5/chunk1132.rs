//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1132/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1132(t19094: f64, t962: f64, t971: f64, t6406: f64, t9630: f64, t9634: f64, t969: f64, t13857: f64, t1694: f64, t18999: f64, t19006: f64, t19011: f64, t19019: f64, t19022: f64, t19042: f64, t19044: f64, t19047: f64, t3001: f64, t4735: f64, t4741: f64, t4760: f64, t4765: f64, t6408: f64, t6425: f64, t6429: f64, t960: f64, t972: f64) -> (f64, f64, f64) {
    let t19096 = t962 * t19094 * t971;
    let t19101 = t9630 * t6406;
    let t19102 = t9634 * t969;
    let t19103 = t19101 * t19102;
    let t19106 = t18999 - 0.17315755899375863299e2_f64 * t3001 * t6429 - 0.11696446794910408142e1_f64 * t4735 * t4760 + 0.11696446794910408142e1_f64 * t3001 * t6408 + 0.23392893589820816284e1_f64 * t960 * t19006 - 0.346315117987517266e2_f64 * t4735 * t4765 - 0.35089340384731224426e1_f64 * t960 * t19011 - 0.58482233974552040708e0_f64 * t3001 * t6425 - 0.11696446794910408142e1_f64 * t13857 * t1694 + 0.1038945353962551798e3_f64 * t960 * t19019 - 0.58482233974552040708e0_f64 * t19022 * t972 - t19042 + 0.11696446794910408142e1_f64 * t960 * t19044 - 0.34631511798751726598e2_f64 * t960 * t19047 - 0.58482233974552040708e0_f64 * t960 * t19096 + 0.23392893589820816284e1_f64 * t4735 * t4741 - 0.1025389702100779493e4_f64 * t960 * t19103;
    (t19096, t19103, t19106)
}
