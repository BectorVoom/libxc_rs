//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 379/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk379<F: Float>(t97: F, t620: F, t34: F, t99: F, t115: F, t681: F) -> (F, F, F, F, F) {
    let t1884 = 1.0 / t97;
    let t1888 = 1.0 / t620;
    let t1889 = t34 * t1888;
    let t1896 = 1.0 / t99;
    let t1916 = t681 * t115;
    (t1884, t1888, t1889, t1896, t1916)
}
