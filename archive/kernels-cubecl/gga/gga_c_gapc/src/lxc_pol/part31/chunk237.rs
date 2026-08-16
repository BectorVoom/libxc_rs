//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 237/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk237<F: Float>(t291: F, t597: F, t906: F, t315: F, t604: F, t181: F, t820: F, t311: F, t825: F) -> (F, F, F, F) {
    let t907 = t597 * t291 * t906;
    let t910 = t604 * t315;
    let t913 = t181 * t820;
    let t916 = t311 * t825;
    (t907, t910, t913, t916)
}
