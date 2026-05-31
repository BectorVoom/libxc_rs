//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3422/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3422<F: Float>(t41441: F, t63462: F, t63464: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t63554: F, t63557: F, t63560: F, t63563: F, t63566: F, t63568: F) -> F {
    let t64310 = -F::cast_from(0.27785333333333333334e0_f64) * t63541 + F::cast_from(0.46308888888888888889e-1_f64) * t63543 - F::cast_from(0.11577222222222222222e0_f64) * t63545 - F::cast_from(0.27785333333333333334e0_f64) * t63547 + F::cast_from(0.92617777777777777779e-1_f64) * t63549 + F::cast_from(0.61745185185185185186e-1_f64) * t63551 + F::cast_from(0.20839e0_f64) * t63554 + F::cast_from(0.55570666666666666666e0_f64) * t63557 - F::cast_from(0.69463333333333333334e-1_f64) * t63560 - F::cast_from(0.46308888888888888889e-1_f64) * t63563 - F::cast_from(0.10805407407407407407e0_f64) * t63566 - F::cast_from(0.3529725e1_f64) * t63568 + F::cast_from(0.61745185185185185184e0_f64) * t41441 + F::cast_from(0.20659e1_f64) * t63462 - F::cast_from(0.22954444444444444444e0_f64) * t63464;
    t64310
}
