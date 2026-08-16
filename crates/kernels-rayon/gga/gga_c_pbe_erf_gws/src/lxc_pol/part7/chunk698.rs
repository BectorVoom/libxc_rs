//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 698/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk698(t481: f64, t510: f64, t5651: f64, t142: f64, t1533: f64, t525: f64, t2030: f64, t520: f64, t2032: f64, t1452: f64, t169: f64, t301: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5652 = t510 * t481;
    let t5653 = t5651 * t5652;
    let t5656 = t142 * t1533;
    let t5657 = t525 * t5656;
    let t5660 = t2030 * t520;
    let t5661 = t5660 * t2032;
    let t5666 = t169 * t784 * t1452 * t301;
    (t5652, t5653, t5656, t5657, t5660, t5661, t5666)
}
