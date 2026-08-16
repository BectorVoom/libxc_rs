//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 891/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk891(t2303: f64, t655: f64, t130: f64, t2289: f64, t675: f64, t146: f64, t2306: f64) -> (f64, f64) {
    let t7938 = 1.0_f64 / t2303 / t655;
    let t7939 = t130 * t7938;
    let t7940 = t2289 * t675;
    let t7942 = 1.0_f64 / t2306 / t146;
    let t7943 = t7940 * t7942;
    let t7945 = 0.51726012919273400301e3_f64 * t7939 * t7943;
    (t7940, t7945)
}
