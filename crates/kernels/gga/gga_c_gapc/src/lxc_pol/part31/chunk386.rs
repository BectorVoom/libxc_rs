//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 386/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk386<F: Float>(t628: F, t641: F, t1554: F, t181: F, t200: F, t505: F, t172: F) -> (F, F, F, F) {
    let t1921 = t628 * t641;
    let t1924 = t181 * t1554;
    let t1927 = t505 * t200;
    let t1928 = t1927 * t172;
    (t1921, t1924, t1927, t1928)
}
