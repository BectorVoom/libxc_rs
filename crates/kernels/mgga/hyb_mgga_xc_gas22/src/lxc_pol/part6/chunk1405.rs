//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1405/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1405<F: Float>(t21940: F, t21955: F, t21957: F, t21962: F, t21965: F, t21969: F, t21973: F, t25959: F, t25962: F, t25964: F, t25966: F, t25968: F, t30390: F, t30392: F, t30399: F, t30402: F, t30404: F, t30406: F) -> F {
    let t30408 = -F::new(0.11696447245269292414e1) * t25959 - F::new(0.36622894612013090108e-3) * t30390 + F::new(0.24415263074675393405e-3) * t30392 + t21940 - F::new(0.36622894612013090108e-3) * t25962 + F::new(0.70178683471615754484e1) * t25964 - F::new(0.10389515463408878255e3) * t25966 - F::new(0.20508037716432813315e4) * t25968 + F::new(0.10843581300301739842e-1) * t30399 - t21955 - F::new(0.11696447245269292414e1) * t21957 - F::new(0.5848223622634646207e0) * t30402 + F::new(0.11696447245269292414e1) * t30404 - t21962 + t21965 + t21969 + t21973 + F::new(8.0) * t30406;
    t30408
}
