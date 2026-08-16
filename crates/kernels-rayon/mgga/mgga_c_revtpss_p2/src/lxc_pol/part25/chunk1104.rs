//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1104/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1104(t13625: f64, t8717: f64, t25082: f64, t1450: f64, t3889: f64, t7237: f64, t2014: f64, t7235: f64, t7316: f64, t1931: f64, t2327: f64, t10301: f64, t6957: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25083 = t8717 * t13625;
    let t25085 = 6.0_f64 * t25082 * t25083;
    let t25089 = t1450 * t3889;
    let t25090 = t7237 * t25089;
    let t25092 = 3.0_f64 * t2014 * t25090;
    let t25095 = 2.0_f64 * t7235 * t7316;
    let t25096 = t1931 * t2327;
    let t25099 = t10301 * t6957;
    (t25083, t25085, t25089, t25090, t25092, t25095, t25096, t25099)
}
