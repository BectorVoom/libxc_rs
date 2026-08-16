//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1580/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1580(t1695: f64, t3075: f64, t1079: f64, t3215: f64, t4858: f64, t372: f64, t4872: f64, t4786: f64, t4873: f64, t11696: f64, t4781: f64, t3092: f64) -> (f64, f64, f64, f64) {
    let t15578 = t1695 * t3075;
    let t15579 = t1079 * t15578;
    let t15583 = 0.28582678745379824648e-3_f64 * t4858 * t3215;
    let t15584 = t372 * t4872;
    let t15585 = t4873 * t4786;
    let t15586 = t15584 * t15585;
    let t15591 = t4781 * t11696;
    let t15592 = t3092 * t15591;
    (t15579, t15583, t15586, t15592)
}
