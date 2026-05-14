//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 343/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk343<F: Float>(t1514: F, t442: F, t441: F, t1136: F, t1239: F, t894: F) -> (F, F, F, F) {
    let t1515 = t442 * t1514;
    let t1516 = t441 * t1515;
    let t1519 = t1136 * t1239;
    let t1520 = t894 * t1519;
    (t1515, t1516, t1519, t1520)
}
