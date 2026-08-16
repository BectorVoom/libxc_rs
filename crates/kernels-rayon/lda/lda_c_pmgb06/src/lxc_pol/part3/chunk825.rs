//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 825/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk825(t110: f64, t2221: f64, t360: f64, t4394: f64, t64: f64, t35: f64, t2226: f64, t947: f64, t3577: f64, t3579: f64, t3601: f64, t3603: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5806 = t110 * t2221;
    let t5808 = t360 * t5806 / 3.0_f64;
    let t5809 = t64 * t4394;
    let t5810 = t35 * t5809;
    let t5813 = t2226 * t947;
    let t5820 = 0.6495611111111111_f64 * t3577;
    let t5821 = 0.48717083333333333_f64 * t3579;
    let t5825 = 0.9743416666666667_f64 * t3601;
    let t5826 = 1.2991222222222223_f64 * t3603;
    (t5806, t5808, t5809, t5810, t5813, t5820, t5821, t5825, t5826)
}
