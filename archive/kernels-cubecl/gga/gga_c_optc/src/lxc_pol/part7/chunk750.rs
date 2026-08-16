//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 750/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk750<F: Float>(t2333: F, t7234: F, t2367: F, t2543: F, t999: F, t6541: F, t769: F) -> (F, F, F, F) {
    let t7235 = t2333 * t7234;
    let t7239 = t2367 * t2543;
    let t7240 = t999 * t7239;
    let t7244 = t769 * t6541;
    (t7235, t7239, t7240, t7244)
}
