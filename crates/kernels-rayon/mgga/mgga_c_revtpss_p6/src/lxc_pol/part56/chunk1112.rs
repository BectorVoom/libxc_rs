//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1112/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1112(t121175: f64, t13847: f64, t1903: f64, t121232: f64, t121174: f64, t25876: f64, t545: f64, t5774: f64, t27864: f64, t8707: f64, t14224: f64, t7301: f64) -> (f64, f64, f64, f64, f64) {
    let t125900 = t13847 * t121175 * t1903;
    let t125901 = t121232 * t125900;
    let t125903 = t121174 * t125900;
    let t125906 = t25876 * t545 * t5774;
    let t125915 = t8707 * t27864;
    let t125918 = t7301 * t14224;
    (t125901, t125903, t125906, t125915, t125918)
}
