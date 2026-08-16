//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 276/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk276(t188: f64, t438: f64, t492: f64, t542: f64, t547: f64, t549: f64, t804: f64, t808: f64, t817: f64, t826: f64, t833: f64, t837: f64, t846: f64, t855: f64, t856: f64) -> f64 {
    let t859 = t804 + t438 + t808 + t817 - t826 + t833 + t492 + t837 + t846 - t855 + 4.0_f64 / 3.0_f64 * t856 * t188 + t542 + t547 + t549;
    t859
}
