//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 865/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk865(t283: f64, t3881: f64, t3884: f64, t3888: f64, t3939: f64, t3944: f64, t3954: f64, t3959: f64, t3962: f64, t3965: f64, t3968: f64, t3970: f64, t4568: f64, t4569: f64, t4570: f64, t4571: f64, t6067: f64) -> f64 {
    let t6102 = t3881 - t3884 - t3888 + 0.0197516734986138_f64 * t6067 * t283 + 12.0_f64 * t3939 - t4568 + t3944 + t4569 - t4570 - t3954 + t4571 - t3959 - t3962 + t3965 + t3968 + 0.01084358130030174_f64 * t3970;
    t6102
}
