//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2263/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2263(t28299: f64, t81979: f64, t28273: f64, t6547: f64, t13042: f64, t17052: f64, t17090: f64, t218: f64, t25170: f64, t25330: f64, t259: f64, t4147: f64, t6632: f64, t7517: f64, t82259: f64, t98876: f64, t98975: f64, t98983: f64, t98986: f64) -> f64 {
    let t98993 = t81979 * t28299;
    let t98995 = t6547 * t28273;
    let t98999 = -12.0_f64 * t98975 * t25170 + 4.0_f64 * t13042 * t7517 + 2.0_f64 * t17052 * t6632 + 0.41123351671205660912e-2_f64 * t98983 - 0.82246703342411321825e-2_f64 * t98986 - 2.0_f64 * t4147 * t25330 + 0.63969658155208805863e-1_f64 * t82259 + 2.0_f64 * t17090 * t6632 - 0.11514538467937585055e0_f64 * t98993 - 0.19190897446562641759e-1_f64 * t98995 + t218 * t98876 * t259;
    t98999
}
