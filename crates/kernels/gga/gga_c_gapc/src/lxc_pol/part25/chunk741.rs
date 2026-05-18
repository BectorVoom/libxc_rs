//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 741/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk741<F: Float>(t1266: F, t996: F, t1001: F, t2902: F, t632: F, t458: F, t998: F, t568: F, t2903: F, t1587: F, t493: F, t2911: F) -> (F, F, F, F, F, F) {
    let t8521 = t996 * t1266;
    let t8522 = t8521 * t1001;
    let t8524 = t2902 * t632;
    let t8525 = t998 * t458;
    let t8526 = t8524 * t8525;
    let t8528 = t998 * t568;
    let t8529 = t2903 * t8528;
    let t8531 = t493 * t1587;
    let t8532 = t2911 * t8531;
    (t8521, t8522, t8524, t8526, t8529, t8532)
}
