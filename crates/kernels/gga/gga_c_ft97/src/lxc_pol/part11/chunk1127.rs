//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1127/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1127<F: Float>(t41451: F, t41456: F, t41461: F, t41466: F, t41471: F, t41475: F, t41480: F, t41484: F, t41488: F, t41492: F, t41495: F, t41512: F) -> (F, F) {
    let t43626 = -F::cast_from(0.62232801019753086422e0_f64) * t41451 + F::cast_from(0.31116400509876543211e0_f64) * t41456 + F::cast_from(0.80013601311111111114e0_f64) * t41461 - F::cast_from(0.80013601311111111114e0_f64) * t41466 + F::cast_from(0.66678001092592592595e-1_f64) * t41471 + F::cast_from(0.8890400145679012346e-1_f64) * t41475 - F::cast_from(0.40006800655555555556e0_f64) * t41480 + F::cast_from(0.60010200983333333334e0_f64) * t41484 - F::cast_from(0.10001700163888888889e0_f64) * t41488 - F::cast_from(0.13335600218518518519e0_f64) * t41492 + F::cast_from(0.44452000728395061732e-1_f64) * t41495;
    let t43631 = F::cast_from(0.4939111192043895748e-1_f64) * t41512;
    (t43626, t43631)
}
