//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 930/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk930(t29516: f64, t7315: f64, t2586: f64, t9065: f64, t741: f64, t11807: f64, t29274: f64, t746: f64, t2560: f64, t9020: f64, t2563: f64, t9054: f64) -> (f64, f64, f64, f64, f64) {
    let t29517 = t7315 * t29516;
    let t29519 = t2586 * t9065;
    let t29520 = t741 * t29519;
    let t29522 = t11807 * t29274;
    let t29523 = t746 * t29522;
    let t29524 = t741 * t29523;
    let t29526 = t2560 * t9020;
    let t29528 = t9054 * t2563;
    (t29517, t29520, t29524, t29526, t29528)
}
