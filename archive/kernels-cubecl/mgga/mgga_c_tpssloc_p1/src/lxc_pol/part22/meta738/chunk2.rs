//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2424/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2424<F: Float>(t68498: F, t68500: F, t68502: F, t68504: F, t68506: F, t68509: F, t68511: F, t68515: F, t68518: F, t68523: F, t68527: F, t68530: F) -> F {
    let t69093 = -F::cast_from(0.103295e1_f64) * t68498 + F::cast_from(0.30872592592592592593e-1_f64) * t68500 + F::cast_from(0.69463333333333333333e-1_f64) * t68502 + F::cast_from(0.41678e0_f64) * t68504 - F::cast_from(0.13892666666666666667e0_f64) * t68506 + F::cast_from(0.794188125e1_f64) * t68509 - F::cast_from(0.473371875e0_f64) * t68511 - F::cast_from(0.187551e1_f64) * t68515 + F::cast_from(0.62517e0_f64) * t68518 + F::cast_from(0.55570666666666666666e0_f64) * t68523 - F::cast_from(0.13892666666666666667e0_f64) * t68527 - F::cast_from(0.10805407407407407407e0_f64) * t68530;
    t69093
}
