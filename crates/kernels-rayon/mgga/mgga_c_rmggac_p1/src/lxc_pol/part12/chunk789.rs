//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 789/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk789(t4601: f64, t7769: f64, t275: f64, t7950: f64, t132: f64, t26078: f64, t36: f64, t4787: f64, t638: f64, t71: f64, t2127: f64, t934: f64) -> (f64, f64, f64, f64) {
    let t36680 = t4601 * t7769;
    let t36689 = t275 * t7950;
    let t36700 = t638 * t36 * t26078 * t71 * t132 * t4787;
    let t36701 = 0.91462949374725084942e-3_f64 * t36700;
    let t36710 = t934 * t2127;
    (t36680, t36689, t36701, t36710)
}
