//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3310/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3310(t10657: f64, t18677: f64, t39694: f64, t39697: f64, t39701: f64, t39719: f64, t39723: f64, t39724: f64, t39726: f64, t4366: f64, t4504: f64, t51320: f64, t51445: f64, t51452: f64, t51460: f64, t5978: f64, t61679: f64, t62612: f64, t62641: f64, t820: f64) -> f64 {
    let t62754 = 0.60712963356159538786e-1_f64 * t51445 + 0.65049603595885220126e-3_f64 * t39694 + t39697 + 0.22089088168956307394e-3_f64 * t51452 + 0.26341796731742046394e1_f64 * t4504 * t62641 * t4366 - 0.39274398764404314548e-3_f64 * t39701 - 0.65854491829355115987e0_f64 * t820 * t10657 * t5978 + 0.26341796731742046394e1_f64 * t4504 * t62612 * t4366 + 0.39274398764404314548e-3_f64 * t39719 - t39723 - 0.43902994552903410656e-1_f64 * t51460 + 0.52039682876708176102e-2_f64 * t39724 - 0.73171657588172351096e-2_f64 * t39726 + 0.15805078039045227836e2_f64 * t51320 * t18677 * t61679;
    t62754
}
