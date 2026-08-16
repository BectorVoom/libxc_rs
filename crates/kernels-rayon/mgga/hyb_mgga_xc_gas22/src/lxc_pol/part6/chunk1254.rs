//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1254/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1254(t2903: f64, t9611: f64, t1166: f64, t15052: f64, t1143: f64, t15041: f64, t2867: f64, t9573: f64, t3687: f64, t7774: f64, t22714: f64, t536: f64, t537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26096 = t2903 * t9611;
    let t26103 = t1166 * t15052;
    let t26113 = t1143 * t15041;
    let t26118 = t2867 * t9573;
    let t26122 = t7774 * t3687;
    let t26158 = t536 * t537 * t22714;
    (t26096, t26103, t26113, t26118, t26122, t26158)
}
