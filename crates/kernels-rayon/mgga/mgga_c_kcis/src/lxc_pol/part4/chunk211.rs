//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 211/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk211(t673: f64, t678: f64, t680: f64, t643: f64, t665: f64, t666: f64, t668: f64, t671: f64) -> (f64, f64) {
    let t681 = t673 * t678 * t680;
    let t684 = t643 + t665 - 0.18311555036753159941e-3_f64 * t666 * t668 - 0.58482233974552040708e0_f64 * t671 * t681;
    (t681, t684)
}
