//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 789/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk789(t38471: f64, t674: f64, t118: f64, t7417: f64, t338: f64, t618: f64, t34760: f64, t8450: f64) -> (f64, f64, f64, f64) {
    let t38472 = t38471 * t674;
    let t38508 = t7417 * t118;
    let t38523 = t338 * t618;
    let t38530 = t8450 * t34760;
    (t38472, t38508, t38523, t38530)
}
