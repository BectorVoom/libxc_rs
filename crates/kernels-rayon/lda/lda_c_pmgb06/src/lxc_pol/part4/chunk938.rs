//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 938/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk938(t5803: f64, t56: f64, t5980: f64, t38: f64, t370: f64, t3577: f64, t3603: f64, t2209: f64, t780: f64, t2715: f64, t342: f64, t2712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7013 = 0.6495611111111111_f64 * t5803;
    let t7015 = t56 * t5980;
    let t7017 = 2.923025_f64 * t38 * t7015;
    let t7018 = t370 * t5980;
    let t7024 = 0.3247805555555556_f64 * t3577;
    let t7026 = 0.6495611111111111_f64 * t3603;
    let t7027 = t780 * t2209;
    let t7031 = t2715 * t342;
    let t7035 = t2712 * t342;
    (t7013, t7015, t7017, t7018, t7024, t7026, t7027, t7031, t7035)
}
