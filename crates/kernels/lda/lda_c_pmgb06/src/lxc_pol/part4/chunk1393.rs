//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1393/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1393<F: Float>(t11920: F, t11928: F, t11930: F, t16006: F, t16008: F, t16011: F, t16014: F, t16018: F, t16021: F, t16023: F, t16026: F, t16028: F, t16030: F, t16032: F, t16034: F) -> F {
    let t18194 = F::cast_from(0.4862222222222222_f64) * t11920 + F::new(2.0) / F::new(3.0) * t11928 + F::cast_from(0.12155555555555556_f64) * t11930 + t16006 + t16008 + t16011 + t16014 - t16018 + t16021 - t16023 - t16026 - t16028 - t16030 - t16032 + t16034;
    t18194
}
