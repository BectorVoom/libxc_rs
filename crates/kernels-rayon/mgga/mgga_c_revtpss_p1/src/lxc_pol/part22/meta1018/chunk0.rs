//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3521/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3521(t1011: f64, t140: f64, t19916: f64, t1668: f64, t372: f64, t4823: f64, t1043: f64, t11249: f64, t11866: f64, t19976: f64, t19907: f64, t3241: f64) -> (f64, f64, f64, f64, f64) {
    let t66686 = t1011 * t140 * t19916;
    let t66689 = t372 * t4823 * t1668;
    let t66702 = t11249 * t1043;
    let t66712 = t11866 * t19976;
    let t66714 = t3241 * t19907;
    (t66686, t66689, t66702, t66712, t66714)
}
