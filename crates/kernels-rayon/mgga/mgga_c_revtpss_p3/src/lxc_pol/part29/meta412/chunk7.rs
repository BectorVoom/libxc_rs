//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1503/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1503(t11922: f64, t4906: f64, t3115: f64, t15957: f64, t4910: f64, t3117: f64, t3075: f64, t357: f64, t4781: f64, t11670: f64, t4890: f64, t3317: f64) -> (f64, f64, f64, f64, f64) {
    let t16035 = t11922 * t4906;
    let t16037 = 0.28582678745379824648e-3_f64 * t3115 * t16035;
    let t16039 = t15957 * t4910;
    let t16040 = t3117 * t16039;
    let t16043 = t357 * t3075;
    let t16044 = t4781 * t16043;
    let t16045 = t3117 * t16044;
    let t16048 = t11670 * t4890;
    let t16049 = t3317 * t16048;
    (t16037, t16040, t16045, t16048, t16049)
}
