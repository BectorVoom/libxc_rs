//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1187/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1187(t1470: f64, t21663: f64, t1497: f64, t5868: f64, t77: f64, t4173: f64, t5826: f64, t1493: f64, t5872: f64, t22742: f64, t84: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114270 = t21663 * t1470;
    let t114288 = t77 * t5868 * t1497;
    let t114296 = t4173 * t5826;
    let t114301 = t77 * t1493 * t5872;
    let t114305 = t77 * t84 * t22742;
    let t114311 = t77 * t84 * t5825;
    (t114270, t114288, t114296, t114301, t114305, t114311)
}
