//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 376/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk376<F: Float>(t1681: F, t1700: F, t1702: F, t295: F, t471: F, t64: F, t719: F, t90: F) -> F {
    let t1710 = t1702 * t471 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t719 * t64 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t1681 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t1700 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t295 * t90;
    t1710
}
