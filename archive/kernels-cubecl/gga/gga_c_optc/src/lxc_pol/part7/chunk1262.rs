//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1262/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1262<F: Float>(t2367: F, t7267: F, t999: F, t23574: F, t2360: F, t24386: F, t24388: F, t24392: F, t24693: F, t24696: F, t24702: F, t2563: F, t26057: F, t277: F, t7268: F, t7301: F, t7304: F, t914: F, t95: F, t962: F) -> F {
    let t26063 = t999 * t2367 * t7267;
    let t26071 = -F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t24386 - F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t24388 + F::cast_from(140.0_f64) / F::cast_from(81.0_f64) * t999 * t914 * t24392 * t23574 + F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t26057 * t962 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t26063 + F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t7304 * t2563 + F::cast_from(4.0_f64) * t2360 * t7268 - t24693 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t2360 * t7301 - t24696 - t24702;
    t26071
}
