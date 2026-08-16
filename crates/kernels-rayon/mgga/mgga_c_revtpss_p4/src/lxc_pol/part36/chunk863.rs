//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 863/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk863(t13126: f64, t460: f64, t12051: f64, t471: f64, t11239: f64, t3596: f64, t3603: f64, t13038: f64, t13045: f64, t1275: f64, t225: f64, t1466: f64, t2246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13127 = t460 * t13126;
    let t13129 = t12051 * t471;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13143 = t12051 * t3603;
    let t13147 = t11239 * t13038;
    let t13148 = t460 * t13147;
    let t13149 = t12051 * t13045;
    let t13180 = t1275 * t1275;
    let t13181 = 1.0_f64 / t13180;
    let t13182 = t225 * t13181;
    let t13272 = t1466 * t2246;
    (t13127, t13129, t13142, t13143, t13148, t13149, t13180, t13181, t13182, t13272)
}
