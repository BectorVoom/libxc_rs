//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 264/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk264<F: Float>(t921: F, t922: F, t402: F, t839: F, t153: F, t155: F, t400: F, t403: F, t917: F) -> (F, F, F) {
    let t923 = t921 * t922;
    let t926 = t402 * t839;
    let t929 = -12.0 * t153 * t923 + 3.0 * t153 * t926 - t155 * t917 + 6.0 * t400 * t403;
    (t923, t926, t929)
}
