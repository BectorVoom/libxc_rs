//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1907/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1907(t268: f64, t557: f64, t6559: f64, t26333: f64, t81326: f64, t22633: f64, t26338: f64, t80650: f64, t1985: f64, t22934: f64, t26193: f64, t16413: f64, t214: f64, t225: f64, t567: f64) -> (f64, f64, f64, f64, f64) {
    let t90607 = t6559 * t557 * t268;
    let t90609 = t90607 * t81326 * t26333;
    let t90612 = t22633 * t80650 * t26338;
    let t90615 = t1985 * t26193 * t22934;
    let t90626 = t1985 * t214 * t16413 * t225 * t567;
    (t90607, t90609, t90612, t90615, t90626)
}
