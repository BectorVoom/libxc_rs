//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1154/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1154(t32283: f64, t33943: f64, t1401: f64, t32194: f64, t32195: f64, t5659: f64, t5673: f64, t121204: f64, t13847: f64, t1883: f64, t121246: f64, t121326: f64) -> (f64, f64, f64, f64) {
    let t125784 = t33943 * t32283;
    let t125785 = t125784 * t1401;
    let t125793 = t32194 * t5673 * t32195 * t5659;
    let t125796 = t13847 * t121204 * t1883;
    let t125797 = t121246 * t125796;
    let t125799 = t121326 * t125796;
    (t125785, t125793, t125797, t125799)
}
