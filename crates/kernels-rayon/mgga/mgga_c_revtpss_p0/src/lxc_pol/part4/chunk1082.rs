//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1082/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1082(t1246: f64, t13068: f64, t247: f64, t3372: f64, t3634: f64, t1261: f64, t3368: f64, t3636: f64, t3647: f64, t3367: f64, t414: f64, t11239: f64, t1243: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13069 = t13068 * t1246;
    let t13085 = t247 * t3634 * t3372;
    let t13086 = t1261 * t13085;
    let t13089 = t247 * t3634 * t3368;
    let t13090 = t1261 * t13089;
    let t13092 = t3647 * t3636;
    let t13099 = 1.0_f64 / t414 / t3367;
    let t13126 = t11239 * t1243;
    (t13069, t13086, t13090, t13092, t13099, t13126)
}
