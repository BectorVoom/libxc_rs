//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2406/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2406<F: Float>(t68498: F, t68500: F, t68502: F, t68504: F, t68506: F, t68509: F, t68511: F, t68515: F, t68518: F, t68523: F, t68527: F, t68530: F) -> F {
    let t68812 = -F::cast_from(0.59793333333333333333e0_f64) * t68498 + F::cast_from(0.2434271604938271605e-1_f64) * t68500 + F::cast_from(0.54771111111111111111e-1_f64) * t68502 + F::cast_from(0.32862666666666666666e0_f64) * t68504 - F::cast_from(0.10954222222222222222e0_f64) * t68506 + F::cast_from(0.427258125e1_f64) * t68509 - F::cast_from(0.230371875e0_f64) * t68511 - F::cast_from(0.147882e1_f64) * t68515 + F::cast_from(0.49294e0_f64) * t68518 + F::cast_from(0.43816888888888888889e0_f64) * t68523 - F::cast_from(0.10954222222222222222e0_f64) * t68527 - F::cast_from(0.85199506172839506175e-1_f64) * t68530;
    t68812
}
