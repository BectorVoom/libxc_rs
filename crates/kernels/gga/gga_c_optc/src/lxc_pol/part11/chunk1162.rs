//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1162/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1162<F: Float>(t4793: F, t2399: F, t4786: F, t23844: F, t2382: F, t23913: F, t39411: F, t49385: F, t49387: F, t56966: F, t56978: F, t56981: F, t56984: F, t57024: F, t57057: F, t57060: F, t57063: F) -> (F, F, F, F, F) {
    let t57065 = t4793 * t4793;
    let t57066 = t2399 * t57065;
    let t57068 = t4786 * t4786;
    let t57069 = t23844 * t57068;
    let t57071 = t2382 * t57065;
    let t57073 = t23913 * t57068;
    let t57086 = -t57057 / 3.0 + 8.0 * t57060 - 12.0 * t56978 + 2.0 * t57063 - 16.0 / 9.0 * t49385 + 8.0 / 3.0 * t49387 + 8.0 / 3.0 * t56981 - 8.0 / 9.0 * t56984 - 8.0 / 9.0 * t39411 - 8.0 * t57024 + 8.0 * t56966;
    (t57066, t57069, t57071, t57073, t57086)
}
