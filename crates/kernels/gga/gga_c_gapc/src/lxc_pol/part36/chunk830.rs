//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 830/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk830<F: Float>(t9927: F, t9928: F, t9926: F, t3402: F, t9253: F, t1038: F, t8140: F, t3787: F) -> (F, F, F, F, F) {
    let t9929 = t9927 * t9928;
    let t9930 = t9926 * t9929;
    let t9932 = t3402 * t9253;
    let t9933 = t1038 * t8140;
    let t9934 = t3787 * t9933;
    (t9929, t9930, t9932, t9933, t9934)
}
