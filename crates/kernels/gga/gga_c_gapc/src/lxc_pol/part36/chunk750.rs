//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 750/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk750<F: Float>(t611: F, t8769: F, t5409: F, t204: F, t474: F, t1970: F, t1266: F, t191: F, t1046: F, t5730: F, t599: F, t596: F) -> (F, F, F, F) {
    let t8921 = t611 * t8769;
    let t8922 = t8921 * t5409;
    let t8926 = t474 * t204;
    let t8927 = t1970 * t8926;
    let t8929 = t1266 * t191;
    let t8930 = t8929 * t1046;
    let t8932 = t5730 * t599;
    let t8933 = t596 * t8932;
    (t8922, t8927, t8930, t8933)
}
