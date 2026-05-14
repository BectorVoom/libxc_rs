//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1112/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1112<F: Float>(t7931: F, t907: F, t2684: F, t2693: F, t7947: F, t902: F, t334: F, t7946: F, t317: F, t2695: F, t2818: F, t2367: F, t7925: F, t930: F, t2704: F, t7845: F) -> (F, F, F, F, F, F, F, F) {
    let t25256 = t7931 * t907;
    let t25260 = t2684 * t2693;
    let t25267 = t902 * t7947;
    let t25277 = 1.0 / t7946 / t334;
    let t25278 = t317 * t25277;
    let t25279 = t2695 * t2695;
    let t25287 = t2818 * t2818;
    let t25297 = t930 * t2367 * t7925;
    let t25302 = t2704 * t7845;
    (t25256, t25260, t25267, t25278, t25279, t25287, t25297, t25302)
}
