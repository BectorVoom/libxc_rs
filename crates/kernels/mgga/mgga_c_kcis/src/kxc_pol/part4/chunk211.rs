//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 211/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk211<F: Float>(t673: F, t678: F, t680: F, t643: F, t665: F, t666: F, t668: F, t671: F) -> (F, F) {
    let t681 = t673 * t678 * t680;
    let t684 = t643 + t665 - 0.18311555036753159941e-3 * t666 * t668 - 0.58482233974552040708e0 * t671 * t681;
    (t681, t684)
}
