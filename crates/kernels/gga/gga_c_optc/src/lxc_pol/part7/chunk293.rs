//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 293/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk293<F: Float>(t289: F, t314: F, t854: F, t860: F, t862: F, t867: F, t874: F, t878: F, t885: F, t891: F, t893: F, t899: F) -> F {
    let t902 = -t854 * t289 / F::new(36.0) + t860 + t862 * t867 / F::new(288.0) + F::cast_from(0.35500316489081544176e-1_f64) * t874 * t878 - F::cast_from(0.14488602482981263091e-1_f64) * t885 * t314 + t891 + F::cast_from(0.18110753103726578864e-2_f64) * t893 * t899;
    t902
}
