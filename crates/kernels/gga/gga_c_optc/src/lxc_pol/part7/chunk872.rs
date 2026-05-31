//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 872/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk872<F: Float>(t2296: F, t2301: F, t2302: F, t2315: F, t350: F, t8333: F, t8335: F, t8338: F, t8345: F, t8346: F, t8349: F, t8376: F, t974: F, t979: F) -> F {
    let t8378 = -F::cast_from(3.0_f64) * t2296 * t2315 + F::cast_from(6.0_f64) * t2301 * t8349 + F::cast_from(6.0_f64) * t8338 * t2302 + t8333 * t350 - F::cast_from(3.0_f64) * t8335 * t979 - F::cast_from(6.0_f64) * t8345 * t8346 - t974 * t8376;
    t8378
}
