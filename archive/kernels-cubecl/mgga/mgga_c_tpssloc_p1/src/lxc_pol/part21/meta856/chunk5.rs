//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3101/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3101<F: Float>(t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F, t64074: F, t64076: F, t64079: F, t64082: F, t64085: F, t64087: F, t64089: F, t64092: F) -> F {
    let t64245 = -F::cast_from(0.13772666666666666667e1_f64) * t63398 - F::cast_from(0.20659e1_f64) * t63400 + F::cast_from(0.309885e1_f64) * t63404 + F::cast_from(0.123954e2_f64) * t63408 + F::cast_from(0.20659e1_f64) * t63412 + F::cast_from(0.57386111111111111112e0_f64) * t63417 - F::cast_from(0.15302962962962962963e1_f64) * t63422 + F::cast_from(0.92617777777777777779e-1_f64) * t64074 + F::cast_from(0.27785333333333333334e0_f64) * t64076 - F::cast_from(0.69463333333333333334e-1_f64) * t64079 - F::cast_from(0.20839e0_f64) * t64082 - F::cast_from(0.125034e1_f64) * t64085 - F::cast_from(0.55570666666666666667e0_f64) * t64087 - F::cast_from(0.83356000000000000001e0_f64) * t64089 + F::cast_from(0.41678e0_f64) * t64092;
    t64245
}
