//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 813/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk813(t125: f64, t4056: f64, t3936: f64, t3938: f64, t3889: f64, t543: f64, t3937: f64, t1386: f64, t2482: f64, t814: f64, t136: f64, t1412: f64) -> (f64, f64, f64, f64) {
    let t9805 = t125 * t4056;
    let t9807 = t3936 * t9805 * t3938;
    let t9810 = t543 * t3889;
    let t9812 = t3936 * t3937 * t9810;
    let t9816 = t2482 * t1386 * t814;
    let t9817 = t1412 * t136;
    (t9807, t9812, t9816, t9817)
}
