//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1023/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1023<F: Float>(t16294: F, t188: F, t1916: F, t16300: F, t16310: F, t6766: F, t3563: F, t4611: F, t16572: F, t714: F, t16433: F, t22892: F, t16429: F, t2007: F, t16373: F, t2030: F) -> (F, F, F, F, F, F, F, F) {
    let t48067 = t188 * t1916 * t16294;
    let t48070 = t188 * t1916 * t16300;
    let t48101 = t16310 * t6766;
    let t48162 = t4611 * t3563;
    let t48183 = t16572 * t714;
    let t48212 = t22892 * t16433;
    let t48214 = t2007 * t16429;
    let t48260 = t2030 * t16373;
    (t48067, t48070, t48101, t48162, t48183, t48212, t48214, t48260)
}
