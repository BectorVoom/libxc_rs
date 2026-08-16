//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 728/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk728(t1911: f64, t5486: f64, t493: f64, t176: f64, t1988: f64, t1826: f64, t4588: f64, t1821: f64, t2549: f64, t529: f64, t1380: f64, t1414: f64, t2389: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6744 = t5486 * t1911;
    let t6746 = 2.0_f64 / 45.0_f64 * t493 * t6744;
    let t6747 = t1988 * t176;
    let t6748 = t6747 * t1826;
    let t6750 = 4.0_f64 / 45.0_f64 * t493 * t6748;
    let t6751 = t4588 * t176;
    let t6752 = t6751 * t1821;
    let t6754 = 2.0_f64 / 27.0_f64 * t493 * t6752;
    let t6755 = t2549 * t529;
    let t6756 = t1380 * t6755;
    let t6758 = t493 * t6756 / 45.0_f64;
    let t6759 = t1414 * t2389;
    (t6744, t6746, t6747, t6748, t6750, t6751, t6752, t6754, t6755, t6756, t6758, t6759)
}
