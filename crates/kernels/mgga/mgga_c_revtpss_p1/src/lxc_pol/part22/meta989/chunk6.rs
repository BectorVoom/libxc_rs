//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3366/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3366<F: Float>(t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F) -> F {
    let t63426 = -F::new(16.0) / F::new(27.0) * t51973 + F::new(56.0) / F::new(81.0) * t51978 + F::new(56.0) / F::new(81.0) * t41361 + F::new(8.0) / F::new(27.0) * t41363 - F::new(40.0) / F::new(27.0) * t63325 + F::new(16.0) / F::new(3.0) * t63328 + F::new(8.0) * t63336 - F::new(8.0) / F::new(9.0) * t63338 + F::new(8.0) / F::new(27.0) * t63340 + F::new(20.0) / F::new(81.0) * t63342 - F::new(10.0) / F::new(27.0) * t63346 - F::new(80.0) / F::new(81.0) * t63351 + F::new(4.0) / F::new(3.0) * t63355;
    t63426
}
