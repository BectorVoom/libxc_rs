//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 601/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk601<F: Float>(t7754: F, t7771: F, t7782: F, t7786: F, t7804: F, t7820: F, t8186: F, t8192: F, t8195: F, t8260: F, t8338: F, t8348: F, t8352: F) -> F {
    let t8354 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t7782 - F::cast_from(2.0_f64) * t7786 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7804 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7820 - t8186 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t8192 + t8195 - F::cast_from(6.0_f64) * t7754 - F::cast_from(2.0_f64) * t7771 - t8260 + t8338 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t8348 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t8352;
    t8354
}
