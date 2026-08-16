//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 872/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk872(t1003: f64, t1004: f64, t1008: f64, t1009: f64, t1011: f64, t1054: f64, t1061: f64, t350: f64, t3725: f64, t3729: f64, t3793: f64, t3803: f64, t666: f64, t667: f64, t682: f64, t8482: f64, t8519: f64, t8522: f64, t8552: f64, t8594: f64, t8598: f64, t8599: f64, t8610: f64, t8621: f64, t8863: f64, t8867: f64, t967: f64, t991: f64, t992: f64) -> f64 {
    let t8887 = -t8482 + t8519 - 3.5089341735807875_f64 * t1054 * t8522 * t682 + 51.94757731704439_f64 * t1061 * t8522 * t967 + 623.3709278045327_f64 * t3803 * t8599 * t967 + 96.49187699215521_f64 * t1009 * t8863 * t1011 - 24.0_f64 * t3793 * t8867 * t667 - 6.0_f64 * t992 * t8863 * t667 + t8552 + t8594 + t8598 - t8610 - 6.609050294782684_f64 * t350 * t1008 * t1003 * t1011 * t666 + 0.41096_f64 * t350 * t991 * t666 * t1004 + 0.13012297560362088_f64 * t350 * t3729 - 1.9263893255070628_f64 * t350 * t3725 - t8621;
    t8887
}
