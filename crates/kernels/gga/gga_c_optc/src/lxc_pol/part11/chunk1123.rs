//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1123/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1123<F: Float>(t3546: F, t4759: F, t108: F, t16287: F, t176: F, t203: F, t729: F, t16294: F, t188: F, t1916: F, t16300: F, t16310: F, t6766: F) -> (F, F, F, F, F) {
    let t48051 = t3546 * t4759;
    let t48058 = t176 * t729 * t16287 * t108 * t203;
    let t48067 = t188 * t1916 * t16294;
    let t48070 = t188 * t1916 * t16300;
    let t48101 = t16310 * t6766;
    (t48051, t48058, t48067, t48070, t48101)
}
