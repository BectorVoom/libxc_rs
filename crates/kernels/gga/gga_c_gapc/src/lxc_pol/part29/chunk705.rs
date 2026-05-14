//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 705/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk705<F: Float>(t204: F, t474: F, t1970: F, t1266: F, t191: F, t1046: F, t5730: F, t599: F, t596: F, t3081: F, t8725: F, t3638: F, t568: F, t590: F, t5581: F, t1043: F, t1976: F) -> (F, F, F, F, F, F, F) {
    let t8926 = t474 * t204;
    let t8927 = t1970 * t8926;
    let t8929 = t1266 * t191;
    let t8930 = t8929 * t1046;
    let t8932 = t5730 * t599;
    let t8933 = t596 * t8932;
    let t8935 = t8725 * t3081;
    let t8937 = t3638 * t568;
    let t8938 = t590 * t8937;
    let t8940 = t5581 * t599;
    let t8941 = t596 * t8940;
    let t8943 = t1043 * t1976;
    (t8927, t8930, t8933, t8935, t8938, t8941, t8943)
}
