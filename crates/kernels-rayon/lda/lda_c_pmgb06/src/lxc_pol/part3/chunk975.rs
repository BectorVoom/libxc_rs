//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 975/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk975(t377: f64, t5829: f64, t1291: f64, t1296: f64, t1309: f64, t2238: f64, t2255: f64, t3622: f64, t3633: f64, t3656: f64, t384: f64, t5831: f64, t5843: f64, t5880: f64, t787: f64, t8396: f64, t8404: f64, t8413: f64) -> f64 {
    let t11535 = t5829 * t377;
    let t11558 = 6.0_f64 * t1296 * t2255 * t1309 + 2.0_f64 * t1296 * t787 * t3656 + 6.0_f64 * t1296 * t5880 * t384 + 24.0_f64 * t8413 * t787 * t3633 - 3.0_f64 * t11535 * t384 - 3.0_f64 * t1291 * t5880 - 3.0_f64 * t5831 * t1309 - t2238 * t3656 - 3.0_f64 * t3622 * t2255 - 18.0_f64 * t8404 * t5843 - t8396 * t787;
    t11558
}
