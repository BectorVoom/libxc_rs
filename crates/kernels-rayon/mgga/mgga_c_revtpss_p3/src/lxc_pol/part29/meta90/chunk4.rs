//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 547/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk547(t1892: f64, t225: f64, t561: f64, t1437: f64, t1883: f64, t546: f64, t1431: f64, t1436: f64, t213: f64, t820: f64) -> (f64, f64, f64) {
    let t1893 = t1892 * t225;
    let t1894 = t1893 * t561;
    let t1897 = t1437 * t1883;
    let t1900 = t546 * t1892;
    let t1903 = -t1431 + t1436 - 0.65854491829355115987e0_f64 * t820 * t1897 + 0.65854491829355115987e0_f64 * t213 * t1900;
    (t1893, t1894, t1903)
}
