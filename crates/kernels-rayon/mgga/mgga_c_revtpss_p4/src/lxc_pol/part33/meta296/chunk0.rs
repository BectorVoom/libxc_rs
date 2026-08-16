//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1286/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1286(t1376: f64, t9789: f64, t235: f64, t4086: f64, t2453: f64, t240: f64, t2712: f64, t3994: f64, t2713: f64, t3951: f64, t3964: f64, t785: f64, t9731: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9791 = 0.11294745624363664198e-6_f64 * t9789 * t1376;
    let t9792 = t4086 * t235;
    let t9793 = t2453 * t9792;
    let t9794 = t2712 * t240;
    let t9795 = t9794 * t3994;
    let t9796 = t9793 * t9795;
    let t9799 = t3964 * t2713 * t3951;
    let t9801 = t9731 * t785;
    (t9791, t9793, t9794, t9795, t9796, t9799, t9801)
}
