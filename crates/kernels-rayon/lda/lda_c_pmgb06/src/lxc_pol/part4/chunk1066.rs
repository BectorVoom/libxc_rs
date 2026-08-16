//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1066/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1066(t122: f64, t5508: f64, t569: f64, t199: f64, t5567: f64, t1135: f64, t868: f64, t107: f64, t1180: f64, t2164: f64, t2786: f64, t902: f64) -> (f64, f64, f64, f64, f64) {
    let t11729 = t122 * t569 * t5508;
    let t11731 = t5567 * t199;
    let t11733 = t1135 * t868;
    let t11744 = t107 * t1180 * t2164;
    let t11747 = t107 * t2786 * t902;
    (t11729, t11731, t11733, t11744, t11747)
}
