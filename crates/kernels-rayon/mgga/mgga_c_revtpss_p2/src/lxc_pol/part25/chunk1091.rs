//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1091/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1091(t11239: f64, t1243: f64, t460: f64, t13043: f64, t487: f64, t12051: f64, t471: f64, t3727: f64, t473: f64, t1214: f64, t3596: f64, t3603: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13126 = t11239 * t1243;
    let t13127 = t460 * t13126;
    let t13128 = t487 * t13043;
    let t13129 = t12051 * t471;
    let t13130 = t13128 * t13129;
    let t13133 = t473 * t3727;
    let t13134 = t13133 * t1214;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13143 = t12051 * t3603;
    (t13127, t13128, t13130, t13134, t13142, t13143)
}
