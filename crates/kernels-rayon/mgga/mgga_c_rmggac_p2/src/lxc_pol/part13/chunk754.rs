//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 754/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk754(t2604: f64, t7779: f64, t674: f64, t7433: f64, t7715: f64, t5542: f64, t7541: f64, t7244: f64, t7469: f64, t108: f64, t4179: f64, t490: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35262 = t2604 * t7779;
    let t35265 = t7433 * t7715 * t674;
    let t35276 = t7541 * t5542;
    let t35277 = t35276 * t674;
    let t35285 = t7244 * t7469;
    let t35311 = t4179 * t108;
    let t35312 = t490 * t35311;
    (t35262, t35265, t35276, t35277, t35285, t35312)
}
