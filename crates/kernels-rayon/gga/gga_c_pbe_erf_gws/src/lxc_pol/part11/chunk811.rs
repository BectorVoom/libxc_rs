//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 811/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk811(t108: f64, t12339: f64, t12345: f64, t12350: f64, t12355: f64, t2538: f64, t2544: f64, t3346: f64, t3354: f64, t476: f64, t478: f64, t726: f64, t728: f64) -> f64 {
    let t13039 = (40.0_f64 / 27.0_f64 * t476 * t12339 + 20.0_f64 / 3.0_f64 * t2538 * t3346 + 4.0_f64 / 3.0_f64 * t726 * t12345 + 40.0_f64 / 27.0_f64 * t478 * t12350 + 20.0_f64 / 3.0_f64 * t2544 * t3354 + 4.0_f64 / 3.0_f64 * t728 * t12355) * t108;
    t13039
}
