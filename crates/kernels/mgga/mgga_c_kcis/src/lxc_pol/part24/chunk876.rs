//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 876/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk876<F: Float>(t19094: F, t962: F, t971: F, t6406: F, t9630: F, t9634: F, t969: F, t13857: F, t1694: F, t18999: F, t19006: F, t19011: F, t19019: F, t19022: F, t19042: F, t19044: F, t19047: F, t3001: F, t4735: F, t4741: F, t4760: F, t4765: F, t6408: F, t6425: F, t6429: F, t960: F, t972: F) -> (F, F, F) {
    let t19096 = t962 * t19094 * t971;
    let t19101 = t9630 * t6406;
    let t19102 = t9634 * t969;
    let t19103 = t19101 * t19102;
    let t19106 = t18999 - F::new(0.17315755899375863299e2) * t3001 * t6429 - F::new(0.11696446794910408142e1) * t4735 * t4760 + F::new(0.11696446794910408142e1) * t3001 * t6408 + F::new(0.23392893589820816284e1) * t960 * t19006 - F::new(0.346315117987517266e2) * t4735 * t4765 - F::new(0.35089340384731224426e1) * t960 * t19011 - F::new(0.58482233974552040708e0) * t3001 * t6425 - F::new(0.11696446794910408142e1) * t13857 * t1694 + F::new(0.1038945353962551798e3) * t960 * t19019 - F::new(0.58482233974552040708e0) * t19022 * t972 - t19042 + F::new(0.11696446794910408142e1) * t960 * t19044 - F::new(0.34631511798751726598e2) * t960 * t19047 - F::new(0.58482233974552040708e0) * t960 * t19096 + F::new(0.23392893589820816284e1) * t4735 * t4741 - F::new(0.1025389702100779493e4) * t960 * t19103;
    (t19096, t19103, t19106)
}
