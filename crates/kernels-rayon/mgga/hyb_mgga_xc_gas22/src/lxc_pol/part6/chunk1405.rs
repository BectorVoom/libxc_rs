//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1405/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1405(t21940: f64, t21955: f64, t21957: f64, t21962: f64, t21965: f64, t21969: f64, t21973: f64, t25959: f64, t25962: f64, t25964: f64, t25966: f64, t25968: f64, t30390: f64, t30392: f64, t30399: f64, t30402: f64, t30404: f64, t30406: f64) -> f64 {
    let t30408 = -0.11696447245269292414e1_f64 * t25959 - 0.36622894612013090108e-3_f64 * t30390 + 0.24415263074675393405e-3_f64 * t30392 + t21940 - 0.36622894612013090108e-3_f64 * t25962 + 0.70178683471615754484e1_f64 * t25964 - 0.10389515463408878255e3_f64 * t25966 - 0.20508037716432813315e4_f64 * t25968 + 0.10843581300301739842e-1_f64 * t30399 - t21955 - 0.11696447245269292414e1_f64 * t21957 - 0.5848223622634646207e0_f64 * t30402 + 0.11696447245269292414e1_f64 * t30404 - t21962 + t21965 + t21969 + t21973 + 8.0_f64 * t30406;
    t30408
}
