//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 907/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk907(t1174: f64, t18454: f64, t11539: f64, t6119: f64, t4889: f64, t4896: f64, t11570: f64, t5392: f64, t1171: f64, t6109: f64, t6011: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18455 = t1174 * t18454;
    let t18457 = t11539 * t6119;
    let t18458 = t1174 * t18457;
    let t18460 = t4889 * t4896;
    let t18469 = t11570 * t5392;
    let t18489 = t6109 * t1171;
    let t18494 = t699 * t6011;
    (t18455, t18457, t18458, t18460, t18469, t18489, t18494)
}
