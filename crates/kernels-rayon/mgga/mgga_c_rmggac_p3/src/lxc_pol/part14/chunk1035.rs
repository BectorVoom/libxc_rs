//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1035/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1035(t1347: f64, t2408: f64, t118: f64, t2001: f64, t352: f64, t38523: f64, t7720: f64, t34884: f64, t9118: f64, t2283: f64, t34881: f64, t2286: f64, t7939: f64) -> (f64, f64, f64, f64, f64) {
    let t41571 = t1347 * t2408;
    let t41576 = t2001 * t118 * t38523 * t352;
    let t41577 = t7720 * t41576;
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    let t41582 = 0.19863479950205658386e-4_f64 * t41581;
    let t41585 = t7939 * t2286;
    (t41571, t41577, t41579, t41582, t41585)
}
