//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 919/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk919<F: Float>(t208: F, t213: F, t4641: F, t4913: F, t83: F, t4076: F, t588: F, t97: F, t4093: F, t138: F, t163: F, t9175: F, t139: F, t3247: F, t1463: F, t1413: F) -> (F, F, F, F, F, F, F, F) {
    let t9478 = t83 * (-0.33530864197530863 * t4641 + 1.8360493827160493 * t4913) * t208 * t213 / 3.0;
    let t9481 = 0.2431111111111111 * t4076 * t97 * t588;
    let t9483 = t4093 * t97 * t588;
    let t9501 = t138 * t9175 * t163;
    let t9502 = 0.01959135802469136 * t9501;
    let t9507 = t139 * t3247;
    let t9508 = t1463 * t1463;
    let t9509 = 1.0 / t9508;
    let t9525 = 1.0 / t1463 / t1413;
    (t9478, t9481, t9483, t9501, t9502, t9507, t9509, t9525)
}
