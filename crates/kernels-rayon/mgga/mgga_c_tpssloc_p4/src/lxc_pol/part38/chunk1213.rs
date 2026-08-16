//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1213/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1213(t11583: f64, t3961: f64, t3449: f64, t11529: f64, t1709: f64, t1174: f64, t1714: f64, t3475: f64, t460: f64, t4934: f64, t3432: f64, t4889: f64) -> (f64, f64, f64, f64) {
    let t15293 = t11583 * t3961;
    let t15294 = t3449 * t15293;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15303 = t1714 * t3475 * t460;
    let t15304 = t4934 * t15303;
    let t15307 = t4889 * t3432;
    (t15294, t15300, t15304, t15307)
}
