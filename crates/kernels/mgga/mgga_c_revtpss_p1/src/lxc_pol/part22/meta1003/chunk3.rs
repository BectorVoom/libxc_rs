//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3421/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3421<F: Float>(t52126: F, t52128: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63519: F, t63522: F, t63525: F, t63528: F, t63531: F, t63533: F, t63536: F, t63538: F) -> F {
    let t64294 = -F::cast_from(0.4630888888888888889e0_f64) * t52126 + F::cast_from(0.61745185185185185187e0_f64) * t52128 + F::cast_from(0.34431666666666666666e0_f64) * t63447 - F::cast_from(0.516475e0_f64) * t63451 - F::cast_from(0.15302962962962962963e0_f64) * t63453 - F::cast_from(0.68863333333333333334e0_f64) * t63457 + F::cast_from(0.45908888888888888889e0_f64) * t63459 + F::cast_from(0.20839e0_f64) * t63519 + F::cast_from(0.20839e0_f64) * t63522 - F::cast_from(0.34731666666666666667e-1_f64) * t63525 - F::cast_from(0.46308888888888888889e-1_f64) * t63528 - F::cast_from(0.104195e0_f64) * t63531 - F::cast_from(0.38590740740740740742e-1_f64) * t63533 - F::cast_from(0.69463333333333333334e-1_f64) * t63536 + F::cast_from(0.23154444444444444445e0_f64) * t63538;
    t64294
}
