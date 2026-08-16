//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 988/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk988(t3182: f64, t828: f64, t2852: f64, t357: f64, t2251: f64, t3093: f64, t3109: f64, t3096: f64, t3091: f64, t1020: f64, t3105: f64, t247: f64, t2862: f64) -> (f64, f64, f64, f64, f64) {
    let t11703 = t828 * t3182;
    let t11704 = t357 * t2852;
    let t11705 = t11704 * t2251;
    let t11706 = t3093 * t11705;
    let t11707 = t11703 * t11706;
    let t11710 = t828 * t3109;
    let t11711 = t11710 * t3096;
    let t11712 = t3091 * t11711;
    let t11714 = t1020 * t3105;
    let t11722 = t247 * t3109 * t2862;
    (t11707, t11711, t11712, t11714, t11722)
}
