//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 947/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk947(t8614: f64, t8589: f64, t8594: f64, t8598: f64, t8603: f64, t8605: f64, t8610: f64, t8612: f64, t8616: f64, t8621: f64, t8626: f64, t8629: f64, t8633: f64, t8637: f64, t8640: f64, t8644: f64) -> f64 {
    let t11083 = 240.0_f64 * t8614;
    let t11085 = t8589 - t8594 - t8598 + t8603 + t8605 + t8610 - t8612 + t11083 + 60.0_f64 * t8616 + t8621 - t8626 - t8629 - t8633 - t8637 + t8640 + t8644;
    t11085
}
