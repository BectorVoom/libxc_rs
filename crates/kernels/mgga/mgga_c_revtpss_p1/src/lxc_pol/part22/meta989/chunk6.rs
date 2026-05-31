//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3366/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3366<F: Float>(t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F) -> F {
    let t63426 = -F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t51973 + F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t51978 + F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t41361 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t41363 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t63325 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t63328 + F::cast_from(8.0_f64) * t63336 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t63338 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t63340 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t63342 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t63346 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t63351 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t63355;
    t63426
}
