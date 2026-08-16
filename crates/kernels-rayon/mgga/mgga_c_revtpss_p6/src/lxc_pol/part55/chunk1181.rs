//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1181/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1181(t121175: f64, t13847: f64, t1903: f64, t121232: f64, t121174: f64, t120980: f64, t1873: f64, t32265: f64, t32269: f64, t125849: f64, t552: f64, t8590: f64) -> (f64, f64, f64, f64, f64) {
    let t125900 = t13847 * t121175 * t1903;
    let t125901 = t121232 * t125900;
    let t125903 = t121174 * t125900;
    let t125922 = t120980 * t1873;
    let t125923 = t32265 * t125922;
    let t125925 = t32269 * t125922;
    let t125928 = t125849 * t8590 * t552;
    (t125901, t125903, t125923, t125925, t125928)
}
