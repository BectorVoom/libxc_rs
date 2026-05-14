//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 497/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk497<F: Float>(t1384: F, t888: F, t874: F, t1382: F, t2641: F, t1326: F, t522: F) -> (F, F, F) {
    let t3631 = t888 * t1384;
    let t3632 = t874 * t3631;
    let t3634 = t2641 * t1382;
    let t3640 = t522 * t1326;
    (t3632, t3634, t3640)
}
