//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 636/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk636<F: Float>(t115: F, t2010: F, t155: F, t2156: F, t635: F, t140: F, t2087: F, t102: F, t95: F, t195: F, t616: F, t2548: F, t322: F) -> (F, F, F, F, F, F, F) {
    let t3491 = t2010 * t115;
    let t3500 = t155 * t2156;
    let t3501 = t3500 * t635;
    let t3519 = t2087 * t140;
    let t3539 = t95 * t102;
    let t3575 = t195 * t616;
    let t3608 = t322 * t2548;
    (t3491, t3500, t3501, t3519, t3539, t3575, t3608)
}
