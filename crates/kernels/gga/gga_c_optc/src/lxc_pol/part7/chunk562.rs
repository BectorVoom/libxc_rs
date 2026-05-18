//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 562/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk562<F: Float>(t2785: F, t914: F, t2586: F, t942: F, t940: F, t284: F, t853: F, t928: F) -> (F, F, F, F) {
    let t2786 = t914 * t2785;
    let t2789 = t2586 * t942;
    let t2790 = t940 * t2789;
    let t2797 = t928 * t853 * t284;
    (t2786, t2789, t2790, t2797)
}
