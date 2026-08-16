//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 677/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk677(t5434: f64, t712: f64, t1903: f64, t708: f64, t1914: f64, t5384: f64, t5387: f64, t5388: f64, t5390: f64, t5397: f64, t5405: f64, t5408: f64, t5410: f64, t5412: f64, t5415: f64, t5417: f64, t5418: f64, t5423: f64, t5429: f64, t5430: f64, t5433: f64) -> f64 {
    let t5436 = 0.2e-20_f64 * t712 * t5434;
    let t5437 = t708 * t1903;
    let t5439 = -t5384 + t5387 + 2.0_f64 / 3.0_f64 * t5388 + 0.2e-20_f64 * t1914 * t5390 + t5397 + t5405 + t5408 + t5410 + t5412 + t5415 + t5417 + 0.36466666666666666665e0_f64 * t5418 + t5423 + t5429 + 4.0_f64 / 3.0_f64 * t5430 + t5433 + t5436 - 2.0_f64 / 9.0_f64 * t5437;
    t5439
}
