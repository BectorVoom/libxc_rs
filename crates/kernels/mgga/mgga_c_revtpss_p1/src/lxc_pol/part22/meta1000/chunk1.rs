//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3399/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3399<F: Float>(t41441: F, t63462: F, t63464: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t63554: F, t63557: F, t63560: F, t63563: F, t63566: F, t63568: F) -> F {
    let t63813 = -F::new(0.22076e0) * t63541 + F::cast_from(0.36793333333333333334e-1_f64) * t63543 - F::cast_from(0.91983333333333333334e-1_f64) * t63545 - F::new(0.22076e0) * t63547 + F::cast_from(0.73586666666666666667e-1_f64) * t63549 + F::cast_from(0.49057777777777777778e-1_f64) * t63551 + F::new(0.16557e0) * t63554 + F::new(0.44152e0) * t63557 - F::new(0.5519e-1) * t63560 - F::cast_from(0.36793333333333333333e-1_f64) * t63563 - F::cast_from(0.8585111111111111111e-1_f64) * t63566 - F::new(0.258925e1) * t63568 + F::cast_from(0.49057777777777777778e0_f64) * t41441 + F::new(0.12077e1) * t63462 - F::cast_from(0.13418888888888888889e0_f64) * t63464;
    t63813
}
