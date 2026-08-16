//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1497/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1497(t11648: f64, t3169: f64, t3133: f64, t373: f64, t1062: f64, t11782: f64, t10356: f64, t11150: f64, t357: f64, t11853: f64, t828: f64, t3229: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42383 = t3169 * t11648;
    let t42385 = t3133 * t3133;
    let t42386 = t373 * t42385;
    let t42391 = t11782 * t1062;
    let t42397 = t357 * t11150 * t10356;
    let t42410 = t828 * t11853;
    let t42415 = t360 * t3229;
    (t42383, t42385, t42386, t42391, t42397, t42410, t42415)
}
