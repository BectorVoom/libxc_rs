//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3640/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3640<F: Float>(t58145: F, t58147: F, t68470: F, t68473: F, t68476: F, t68479: F, t68481: F, t68484: F, t68486: F, t68488: F, t68490: F, t68493: F, t68495: F, t68497: F) -> F {
    let t68887 = -F::cast_from(0.3560484375e1_f64) * t68470 + F::cast_from(0.142419375e1_f64) * t68473 + F::cast_from(0.1151859375e0_f64) * t68476 - F::new(0.76790625e-1) * t68479 - F::new(0.1898925e1) * t68481 - F::new(0.1898925e1) * t68484 - F::new(0.9494625e0) * t68486 - F::new(0.76790625e-1) * t68488 + F::new(0.3071625e0) * t68490 + F::new(0.3071625e0) * t68493 + F::new(0.15358125e0) * t68495 + F::cast_from(0.142419375e1_f64) * t68497 + F::cast_from(0.36514074074074074074e0_f64) * t58145 - F::cast_from(0.10954222222222222222e0_f64) * t58147;
    t68887
}
