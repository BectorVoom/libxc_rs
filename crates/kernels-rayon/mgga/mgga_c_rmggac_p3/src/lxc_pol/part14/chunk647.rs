//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 647/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk647(t2350: f64, t352: f64, t262: f64, t7192: f64, t22: f64, t511: f64, t899: f64, t2347: f64, t321: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8635 = t2350 * t352;
    let t8636 = t262 * t8635;
    let t8637 = t7192 * t8636;
    let t8639 = t511 * t22;
    let t8640 = t899 * t8639;
    let t8641 = t2347 * t321;
    let t8642 = t262 * t8641;
    let t8643 = t8640 * t8642;
    let t8645 = t2347 * t333;
    (t8635, t8636, t8637, t8639, t8640, t8641, t8642, t8643, t8645)
}
