//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 508/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk508<F: Float>(t192: F, t5053: F, t743: F, t2481: F, t3908: F, t3925: F, t462: F, t5099: F, t5102: F, t5106: F, t5110: F, t5114: F, t92: F) -> (F, F) {
    let t5118 = t192 * t743 * t5053;
    let t5120 = t2481 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3908 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3925 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t462 * t5099 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t5102 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t5106 - t462 * t5110 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t92 * t5114 - t92 * t5118;
    (t5118, t5120)
}
