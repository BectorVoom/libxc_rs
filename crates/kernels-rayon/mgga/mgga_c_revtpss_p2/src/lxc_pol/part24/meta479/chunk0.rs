//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1468/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1468(t2439: f64, t6467: f64, t6464: f64, t6461: f64, t3383: f64, t6433: f64, t3432: f64, t3520: f64, t6513: f64, t3495: f64, t3476: f64, t6481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t68583 = t2439 * t6467;
    let t68585 = t2439 * t6464;
    let t68590 = t2439 * t6461;
    let t68792 = t6433 * t3383;
    let t68952 = t6433 * t3432;
    let t69359 = t6513 * t3520;
    let t69371 = t6513 * t3495;
    let t69376 = t6481 * t3476;
    (t68583, t68585, t68590, t68792, t68952, t69359, t69371, t69376)
}
