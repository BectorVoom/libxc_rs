//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 860/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk860(t16243: f64, t482: f64, t11500: f64, t1345: f64, t1357: f64, t16092: f64, t16093: f64, t16100: f64, t16105: f64, t16108: f64, t16112: f64, t16117: f64, t16119: f64, t16122: f64, t16124: f64, t16126: f64, t16226: f64, t1921: f64, t3921: f64, t3940: f64, t3948: f64, t45: f64, t5590: f64) -> (f64, f64) {
    let t16244 = t16243 * t482;
    let t16249 = -0.17315755899375863299e2_f64 * t5590 * t3948 - t16092 - 0.11696446794910408142e1_f64 * t16093 * t1357 - 0.58482233974552040708e0_f64 * t11500 * t1921 + 0.11696446794910408142e1_f64 * t5590 * t3921 + 0.11696446794910408142e1_f64 * t1345 * t16100 - t16105 - 0.1025389702100779493e4_f64 * t1345 * t16108 - 0.34631511798751726598e2_f64 * t1345 * t16112 + t16117 + t16119 + t16122 + t16124 + t16126 + t16226 + 0.19751789702565206229e-1_f64 * t45 * t16244 - 0.58482233974552040708e0_f64 * t5590 * t3940;
    (t16244, t16249)
}
