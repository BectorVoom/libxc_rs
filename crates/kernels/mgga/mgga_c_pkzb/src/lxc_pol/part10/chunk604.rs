//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 604/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk604<F: Float>(t179: F, t2405: F, t824: F, t404: F, t344: F) -> (F, F, F, F) {
    let t2407 = t179 * t2405 * t824;
    let t2408 = t404 * t2407;
    let t2410 = t344 * t344;
    let t2411 = 1.0 / t2410;
    (t2407, t2408, t2410, t2411)
}
