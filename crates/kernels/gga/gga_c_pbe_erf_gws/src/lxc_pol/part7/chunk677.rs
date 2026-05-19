//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 677/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk677<F: Float>(t5434: F, t712: F, t1903: F, t708: F, t1914: F, t5384: F, t5387: F, t5388: F, t5390: F, t5397: F, t5405: F, t5408: F, t5410: F, t5412: F, t5415: F, t5417: F, t5418: F, t5423: F, t5429: F, t5430: F, t5433: F) -> F {
    let t5436 = F::new(0.2e-20) * t712 * t5434;
    let t5437 = t708 * t1903;
    let t5439 = -t5384 + t5387 + F::new(2.0) / F::new(3.0) * t5388 + F::new(0.2e-20) * t1914 * t5390 + t5397 + t5405 + t5408 + t5410 + t5412 + t5415 + t5417 + F::cast_from(0.36466666666666666665e0_f64) * t5418 + t5423 + t5429 + F::new(4.0) / F::new(3.0) * t5430 + t5433 + t5436 - F::new(2.0) / F::new(9.0) * t5437;
    t5439
}
