//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1524/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1524(t2240: f64, t602: f64, t2246: f64, t599: f64, t88: f64, t89: f64, t90: f64, t29: f64, t2248: f64, t644: f64, t2315: f64, t606: f64, t70: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10298 = t2240 * t602;
    let t10301 = t599 * t2246;
    let t10308 = 1.0_f64 / t90 / t89 / t88;
    let t10309 = t29 * t10308;
    let t10310 = t2248 * t644;
    let t10313 = t644 * t2315;
    let t10317 = t606 * t70 * t72;
    (t10298, t10301, t10308, t10309, t10310, t10313, t10317)
}
