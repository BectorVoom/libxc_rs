//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1128/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1128<F: Float>(t16243: F, t482: F, t11500: F, t1345: F, t1357: F, t16092: F, t16093: F, t16100: F, t16105: F, t16108: F, t16112: F, t16117: F, t16119: F, t16122: F, t16124: F, t16126: F, t16226: F, t1921: F, t3921: F, t3940: F, t3948: F, t45: F, t5590: F) -> (F, F) {
    let t16244 = t16243 * t482;
    let t16249 = -0.17315755899375863299e2 * t5590 * t3948 - t16092 - 0.11696446794910408142e1 * t16093 * t1357 - 0.58482233974552040708e0 * t11500 * t1921 + 0.11696446794910408142e1 * t5590 * t3921 + 0.11696446794910408142e1 * t1345 * t16100 - t16105 - 0.1025389702100779493e4 * t1345 * t16108 - 0.34631511798751726598e2 * t1345 * t16112 + t16117 + t16119 + t16122 + t16124 + t16126 + t16226 + 0.19751789702565206229e-1 * t45 * t16244 - 0.58482233974552040708e0 * t5590 * t3940;
    (t16244, t16249)
}
