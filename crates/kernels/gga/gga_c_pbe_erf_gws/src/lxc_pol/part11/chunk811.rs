//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 811/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk811<F: Float>(t108: F, t12339: F, t12345: F, t12350: F, t12355: F, t2538: F, t2544: F, t3346: F, t3354: F, t476: F, t478: F, t726: F, t728: F) -> F {
    let t13039 = (F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t476 * t12339 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t2538 * t3346 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t726 * t12345 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t478 * t12350 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t2544 * t3354 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t728 * t12355) * t108;
    t13039
}
