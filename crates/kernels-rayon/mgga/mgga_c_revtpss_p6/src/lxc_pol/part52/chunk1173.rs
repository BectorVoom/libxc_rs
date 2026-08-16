//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1173/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1173(t121204: f64, t13847: f64, t1883: f64, t121246: f64, t121326: f64, t32195: f64, t32206: f64, t5627: f64, t9955: f64, t125587: f64, t32211: f64, t3936: f64) -> (f64, f64, f64, f64) {
    let t125796 = t13847 * t121204 * t1883;
    let t125797 = t121246 * t125796;
    let t125799 = t121326 * t125796;
    let t125803 = t32206 * t9955 * t32195 * t5627;
    let t125807 = t32206 * t3936 * t32211 * t125587;
    (t125797, t125799, t125803, t125807)
}
