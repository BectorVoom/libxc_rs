//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 894/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk894(t15163: f64, t34735: f64, t14314: f64, t551: f64, t262: f64, t7204: f64, t1587: f64, t3080: f64, t2367: f64, t7778: f64, t739: f64, t14174: f64, t6355: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76046 = t34735 * t15163;
    let t76048 = t14314 * t551;
    let t76049 = t262 * t76048;
    let t76050 = t7204 * t76049;
    let t76052 = t3080 * t1587;
    let t76053 = t262 * t76052;
    let t76054 = t7204 * t76053;
    let t76062 = t7778 * t2367;
    let t76063 = t739 * t76062;
    let t76064 = 0.79828278012425390427e-1_f64 * t76063;
    let t76066 = t6355 * t14174;
    (t76046, t76048, t76049, t76050, t76052, t76053, t76054, t76062, t76064, t76066)
}
