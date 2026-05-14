//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 565/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk565<F: Float>(t3099: F, t72: F, t51: F, t6: F, t938: F, t398: F, t58: F, t401: F, t428: F, t22591: F, t379: F, t930: F, t22585: F, t420: F, t423: F, t373: F, t920: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25670 = t72 * t3099;
    let t25675 = t938 * t6 * t51;
    let t25676 = t25675 * t398;
    let t25679 = t58 * t938;
    let t25680 = t25679 * t401;
    let t25684 = t25679 * t428;
    let t25685 = t22591 * t25684;
    let t25688 = t930 * t379;
    let t25689 = t22585 * t25688;
    let t25692 = t420 * t423;
    let t25693 = t920 * t373;
    (t25670, t25676, t25680, t25684, t25685, t25688, t25689, t25692, t25693)
}
