//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 822/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk822<F: Float>(t10188: F, t13699: F, t13701: F, t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16650: F, t7713: F, t232: F, t1342: F, t4815: F, t2373: F, t7609: F) -> (F, F, F, F, F) {
    let t16652 = -t7713 - 0.23744444444444444444e-1 * t10188 + 0.11872222222222222222e-1 * t13699 - 0.35616666666666666666e-1 * t13701 + 0.17808333333333333333e-1 * t13703 - 0.19787037037037037037e-1 * t16630 + 0.71233333333333333332e-1 * t16634 - 0.35616666666666666666e-1 * t16638 - 0.10685e0 * t16642 + 0.10685e0 * t16646 - 0.17808333333333333333e-1 * t16650;
    let t16654 = 0.62182e-1 * t16652 * t232;
    let t16655 = t4815 * t1342;
    let t16657 = 6.0 * t2373 * t16655;
    let t16671 = -t7609 - 0.12361111111111111111e-1 * t10188 + 0.61805555555555555556e-2 * t13699 - 0.18541666666666666667e-1 * t13701 + 0.92708333333333333334e-2 * t13703 - 0.10300925925925925926e-1 * t16630 + 0.37083333333333333333e-1 * t16634 - 0.18541666666666666666e-1 * t16638 - 0.55625000000000000001e-1 * t16642 + 0.55625000000000000001e-1 * t16646 - 0.92708333333333333333e-2 * t16650;
    (t16652, t16654, t16655, t16657, t16671)
}
