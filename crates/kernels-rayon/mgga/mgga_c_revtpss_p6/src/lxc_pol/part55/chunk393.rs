//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 393/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk393(t1949: f64, t225: f64, t257: f64, t209: f64, t785: f64) -> (f64, f64, f64, f64) {
    let t1950 = t1949 * t225;
    let t1951 = t1950 * t257;
    let t1954 = t209 * t209;
    let t1955 = t1954 * t785;
    (t1950, t1951, t1954, t1955)
}
