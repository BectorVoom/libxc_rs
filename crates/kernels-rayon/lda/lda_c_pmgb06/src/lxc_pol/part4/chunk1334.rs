//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1334/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1334(t2623: f64, t3457: f64, t1602: f64, t1992: f64, t493: f64, t27: f64, t545: f64, t7209: f64, t7179: f64, t1377: f64, t2676: f64, t97: f64) -> (f64, f64, f64, f64) {
    let t17538 = t3457 * t2623;
    let t17542 = t493 * t1992 * t17538 * t1602 / 5.0_f64;
    let t17544 = t7209 * t27 * t545;
    let t17547 = t7179 * t27 * t545;
    let t17550 = t2676 * t97 * t1377;
    (t17542, t17544, t17547, t17550)
}
