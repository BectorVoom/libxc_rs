//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 974/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk974<F: Float>(t172: F, t1928: F, t3314: F, t622: F, t3313: F, t176: F, t729: F, t3315: F, t108: F, t616: F, t110: F, t131: F, t2020: F) -> (F, F, F, F, F) {
    let t9361 = t1928 * t172;
    let t9411 = t3314 * t622;
    let t9412 = t3313 * t9411;
    let t9415 = t176 * t729;
    let t9416 = t9415 * t3315;
    let t9546 = t616 * t108;
    let t9547 = t9546 * t110;
    let t9548 = t3313 * t9547;
    let t9598 = t2020 * t131;
    (t9361, t9412, t9416, t9548, t9598)
}
