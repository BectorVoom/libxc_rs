//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 961/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk961(t23958: f64, t341: f64, t225: f64, t366: f64, t1651: f64, t6258: f64, t247: f64, t3116: f64, t1066: f64, t23474: f64, t11853: f64, t23470: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23959 = t23958 * t341;
    let t23960 = t23959 * t225;
    let t23961 = t23960 * t366;
    let t23964 = t1651 * t6258;
    let t23966 = t247 * t3116 * t23964;
    let t23976 = t247 * t1066 * t23474;
    let t23980 = t247 * t11853 * t23470;
    (t23959, t23961, t23964, t23966, t23976, t23980)
}
