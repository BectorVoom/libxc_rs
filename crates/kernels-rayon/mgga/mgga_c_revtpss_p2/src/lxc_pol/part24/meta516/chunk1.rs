//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1537/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1537(t1011: f64, t15987: f64, t23503: f64, t19773: f64, t4845: f64, t140: f64, t23868: f64, t11922: f64, t23930: f64, t4892: f64, t11710: f64, t23903: f64, t4899: f64) -> (f64, f64, f64, f64, f64) {
    let t79944 = t1011 * t15987 * t23503;
    let t79946 = t19773 * t4845;
    let t79957 = t1011 * t140 * t23868;
    let t80038 = t4892 * t11922 * t23930;
    let t80113 = t4899 * t11710 * t23903;
    (t79944, t79946, t79957, t80038, t80113)
}
