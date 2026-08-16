//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 522/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk522(t664: f64, t7778: f64, t739: f64, t2046: f64, t2165: f64, t3047: f64, t2169: f64, t3056: f64, t3057: f64, t14161: f64, t1968: f64, t1966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14207 = t7778 * t664;
    let t14208 = t739 * t14207;
    let t14211 = t2046 * t3047 * t2165;
    let t14214 = t2046 * t3047 * t2169;
    let t14217 = t3056 * t3057 * t2165;
    let t14220 = t3056 * t3057 * t2169;
    let t14224 = t14161 * t1968;
    let t14225 = t1966 * t14224;
    (t14207, t14208, t14211, t14214, t14217, t14220, t14224, t14225)
}
