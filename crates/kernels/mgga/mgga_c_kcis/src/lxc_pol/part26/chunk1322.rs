//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1322/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1322<F: Float>(t102664: F, t1394: F, t7924: F, t21972: F, t303: F, t553: F, t102649: F, t102653: F, t102655: F, t102658: F, t102661: F, t98804: F, t98806: F, t98822: F, t98830: F, t99615: F) -> (F, F, F) {
    let t102666 = t1394 * t102664 * t7924;
    let t102669 = t303 * t553 * t21972;
    let t102671 = t99615 + F::cast_from(0.77382407407407407407e-3_f64) * t98804 - F::cast_from(0.51588271604938271603e-3_f64) * t98806 - F::cast_from(0.11607361111111111111e-2_f64) * t102649 - F::cast_from(0.61905925925925925925e-2_f64) * t98822 - F::cast_from(0.34822083333333333332e-2_f64) * t102653 + F::cast_from(0.61905925925925925924e-2_f64) * t102655 + F::cast_from(0.92858888888888888886e-2_f64) * t102658 - F::cast_from(0.61905925925925925924e-2_f64) * t102661 - F::cast_from(0.51588271604938271603e-3_f64) * t98830 + F::cast_from(0.11349419753086419753e-1_f64) * t102666 + F::cast_from(0.11607361111111111111e-2_f64) * t102669;
    (t102666, t102669, t102671)
}
