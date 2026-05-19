//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 243/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk243<F: Float>(t522: F, t531: F, t182: F, t179: F) -> (F, F, F, F, F) {
    let t720 = -F::cast_from(0.19388333333333333333e1_f64) * t522 - F::new(0.12315e-2) * t531;
    let t722 = t182 * t182;
    let t723 = F::new(1.0) / t722;
    let t724 = t179 * t723;
    let t727 = -F::cast_from(0.72691666666666666667e3_f64) * t522 - F::cast_from(0.78666666666666666667e2_f64) * t531;
    (t720, t722, t723, t724, t727)
}
