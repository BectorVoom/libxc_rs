//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 807/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk807(t15338: f64, t4904: f64, t3447: f64, t3431: f64, t6126: f64, t1174: f64, t6130: f64, t11539: f64, t6119: f64, t4889: f64, t4896: f64, t11570: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18446 = t15338 * t4904;
    let t18447 = t3447 * t18446;
    let t18451 = t3431 * t6126;
    let t18452 = t1174 * t18451;
    let t18454 = t3431 * t6130;
    let t18455 = t1174 * t18454;
    let t18457 = t11539 * t6119;
    let t18458 = t1174 * t18457;
    let t18460 = t4889 * t4896;
    let t18469 = t11570 * t5392;
    (t18447, t18452, t18455, t18458, t18460, t18469)
}
