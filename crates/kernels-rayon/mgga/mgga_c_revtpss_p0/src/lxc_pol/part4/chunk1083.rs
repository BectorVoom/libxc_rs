//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1083/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1083(t13126: f64, t460: f64, t3727: f64, t473: f64, t11239: f64, t3596: f64, t13038: f64, t1269: f64, t3555: f64, t1275: f64, t225: f64, t575: f64, t5789: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13127 = t460 * t13126;
    let t13133 = t473 * t3727;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13147 = t11239 * t13038;
    let t13148 = t460 * t13147;
    let t13177 = t3555 * t1269;
    let t13180 = t1275 * t1275;
    let t13181 = 1.0_f64 / t13180;
    let t13182 = t225 * t13181;
    let t13254 = 2.0_f64 * t5789 * t575;
    (t13127, t13133, t13141, t13142, t13147, t13148, t13177, t13182, t13254)
}
