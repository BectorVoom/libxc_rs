//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1120/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1120(t121126: f64, t32206: f64, t5673: f64, t5727: f64, t25924: f64, t121174: f64, t125662: f64, t124: f64, t1380: f64, t1903: f64, t800: f64, t32705: f64) -> (f64, f64, f64, f64, f64) {
    let t125819 = t32206 * t5673 * t121126 * t5727;
    let t125821 = t25924 * t5727;
    let t125826 = t121174 * t125662;
    let t125830 = t1380 * t800 * t124 * t1903;
    let t125831 = t32705 * t125830;
    (t125819, t125821, t125826, t125830, t125831)
}
