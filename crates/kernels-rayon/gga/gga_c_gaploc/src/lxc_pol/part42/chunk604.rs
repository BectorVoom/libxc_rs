//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 604/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk604(t11470: f64, t568: f64, t11219: f64, t531: f64, t11218: f64, t189: f64, t188: f64, t3565: f64, t524: f64, t1628: f64, t3595: f64, t3591: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11471 = t568 * t11470;
    let t11476 = t531 * t11219;
    let t11481 = t189 * t11218;
    let t11482 = t188 * t11481;
    let t11485 = t524 * t3565;
    let t11490 = t1628 * t3595;
    let t11493 = t1628 * t3591;
    (t11471, t11476, t11481, t11482, t11485, t11490, t11493)
}
