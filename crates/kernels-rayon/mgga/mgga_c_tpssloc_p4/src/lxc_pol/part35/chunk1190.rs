//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1190/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1190(t28142: f64, t6637: f64, t22685: f64, t1799: f64, t26395: f64, t6888: f64, t6415: f64, t6987: f64, t1336: f64, t1814: f64, t2013: f64, t22693: f64, t26381: f64, t26427: f64, t27082: f64, t27088: f64, t28132: f64, t28136: f64, t28140: f64, t6378: f64, t7747: f64) -> (f64, f64, f64, f64, f64) {
    let t28143 = t6637 * t28142;
    let t28144 = t22685 * t28143;
    let t28148 = t26395 * t1799;
    let t28149 = t6637 * t28148;
    let t28150 = t6888 * t28149;
    let t28152 = t6987 * t6415;
    let t28155 = 0.76763589786250567036e-1_f64 * t26381 - t22693 + t6378 * t2013 + t27082 + 0.3289868133696452873e-1_f64 * t28132 + 0.16449340668482264365e-1_f64 * t28136 + t27088 - 0.16449340668482264365e-1_f64 * t28140 + 0.49348022005446793095e-1_f64 * t28144 + 2.0_f64 * t1814 * t7747 - 0.3289868133696452873e-1_f64 * t28150 - t1336 * t28152 + 0.82246703342411321824e-2_f64 * t26427;
    (t28143, t28148, t28149, t28152, t28155)
}
