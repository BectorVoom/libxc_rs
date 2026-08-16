//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 958/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk958(t31092: f64, t6914: f64, t22751: f64, t31145: f64, t22916: f64, t31137: f64, t6888: f64, t22685: f64, t22686: f64, t22724: f64, t31104: f64, t1377: f64, t6992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114208 = t6914 * t31092;
    let t114209 = 0.15352717957250113407e0_f64 * t114208;
    let t114216 = t22751 * t31145;
    let t114217 = 0.15352717957250113407e0_f64 * t114216;
    let t114220 = 0.3289868133696452873e-1_f64 * t6888 * t31137 * t22916;
    let t114223 = 0.9869604401089358619e-1_f64 * t22685 * t31137 * t22686;
    let t114225 = 0.52089578783527170489e-1_f64 * t22724 * t31104;
    let t114226 = t1377 * t6992;
    (t114209, t114217, t114220, t114223, t114225, t114226)
}
