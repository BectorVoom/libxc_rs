//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1322/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1322(t102664: f64, t1394: f64, t7924: f64, t21972: f64, t303: f64, t553: f64, t102649: f64, t102653: f64, t102655: f64, t102658: f64, t102661: f64, t98804: f64, t98806: f64, t98822: f64, t98830: f64, t99615: f64) -> (f64, f64, f64) {
    let t102666 = t1394 * t102664 * t7924;
    let t102669 = t303 * t553 * t21972;
    let t102671 = t99615 + 0.77382407407407407407e-3_f64 * t98804 - 0.51588271604938271603e-3_f64 * t98806 - 0.11607361111111111111e-2_f64 * t102649 - 0.61905925925925925925e-2_f64 * t98822 - 0.34822083333333333332e-2_f64 * t102653 + 0.61905925925925925924e-2_f64 * t102655 + 0.92858888888888888886e-2_f64 * t102658 - 0.61905925925925925924e-2_f64 * t102661 - 0.51588271604938271603e-3_f64 * t98830 + 0.11349419753086419753e-1_f64 * t102666 + 0.11607361111111111111e-2_f64 * t102669;
    (t102666, t102669, t102671)
}
