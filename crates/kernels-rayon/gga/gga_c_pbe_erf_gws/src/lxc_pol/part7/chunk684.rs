//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 684/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk684(t1623: f64, t5493: f64, t1620: f64, t1624: f64, t4913: f64, t256: f64, t5443: f64, t5445: f64, t5449: f64, t5452: f64, t5458: f64, t5460: f64, t5462: f64, t5466: f64, t5469: f64, t5472: f64, t5474: f64, t5476: f64, t5479: f64, t5483: f64, t5487: f64, t5492: f64) -> (f64, f64, f64, f64) {
    let t5494 = t5493 * t1623;
    let t5495 = t1620 * t5494;
    let t5496 = 16.0_f64 / 15.0_f64 * t5495;
    let t5498 = 8.0_f64 / 5.0_f64 * t4913 * t1624;
    let t5499 = -t5443 + t5445 * t256 / 3.0_f64 + t5449 + 0.18233333333333333333e0_f64 * t5452 + t5458 + t5460 - t5462 - t5466 + t5469 + t5472 + t5474 + t5476 + t5479 + t5483 + t5487 + t5492 - t5496 - t5498;
    (t5494, t5496, t5498, t5499)
}
