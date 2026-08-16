//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1185/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1185<F: Float>(t2586: F, t7858: F, t893: F, t22028: F, t894: F, t897: F, t2663: F, t140: F, t305: F) -> (F, F, F, F, F) {
    let t24555 = t2586 * t7858;
    let t24556 = t893 * t24555;
    let t24559 = t894 * t897 * t22028;
    let t24562 = t2663 * t2663;
    let t24563 = F::cast_from(1.0_f64) / t24562;
    let t24565 = t305 * t24563 * t140;
    (t24555, t24556, t24559, t24563, t24565)
}
