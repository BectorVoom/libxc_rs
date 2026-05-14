//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1181/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1181<F: Float>(t1014: F, t29386: F, t28524: F, t303: F, t5633: F, t1983: F, t5757: F, t576: F, t7052: F, t1394: F, t7924: F, t21972: F, t553: F, t102649: F, t102653: F, t98804: F, t98806: F, t98822: F, t98830: F, t99615: F) -> (F, F, F, F, F, F) {
    let t102655 = t1014 * t29386;
    let t102658 = t303 * t28524 * t5633;
    let t102661 = t303 * t1983 * t5757;
    let t102664 = t576 * t7052;
    let t102666 = t1394 * t102664 * t7924;
    let t102669 = t303 * t553 * t21972;
    let t102671 = t99615 + 0.77382407407407407407e-3 * t98804 - 0.51588271604938271603e-3 * t98806 - 0.11607361111111111111e-2 * t102649 - 0.61905925925925925925e-2 * t98822 - 0.34822083333333333332e-2 * t102653 + 0.61905925925925925924e-2 * t102655 + 0.92858888888888888886e-2 * t102658 - 0.61905925925925925924e-2 * t102661 - 0.51588271604938271603e-3 * t98830 + 0.11349419753086419753e-1 * t102666 + 0.11607361111111111111e-2 * t102669;
    (t102655, t102658, t102661, t102666, t102669, t102671)
}
