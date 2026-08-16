//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3254/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3254(t14663: f64, t2745: f64, t40455: f64, t40473: f64, t40475: f64, t40477: f64, t40489: f64, t4364: f64, t4365: f64, t50472: f64, t50493: f64, t50497: f64, t50502: f64, t50504: f64) -> f64 {
    let t61748 = 0.40015750243531754508e-2_f64 * t50472 - 0.42874018118069736972e-3_f64 * t2745 * t4364 * t4365 * t14663 - 0.16065646176094875955e-5_f64 * t40455 - 0.76220476654346199061e-4_f64 * t40473 - 0.76220476654346199061e-4_f64 * t40475 + 0.54208002996571016772e-3_f64 * t40477 + 0.14450132032386466905e-2_f64 * t40489 - 0.28582678745379824648e-4_f64 * t50493 + 0.85748036236139473944e-4_f64 * t50497 + 0.28582678745379824648e-3_f64 * t50502 - 0.30488190661738479624e-3_f64 * t50504;
    t61748
}
