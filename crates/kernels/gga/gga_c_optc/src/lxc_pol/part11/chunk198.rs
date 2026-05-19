//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 198/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk198<F: Float>(t520: F, t522: F, t526: F, t531: F) -> F {
    let t533 = -F::new(0.632975e0) * t520 - F::cast_from(0.29896666666666666667e0_f64) * t522 - F::new(0.1023875e0) * t526 - F::cast_from(0.82156666666666666667e-1_f64) * t531;
    t533
}
