//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 335/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk335(t107: f64, t1180: f64, t395: f64, t93: f64, t944: f64, t955: f64) -> f64 {
    let t1183 = -t93 * t944 / 9.0_f64 + 0.022758333333333332_f64 * t955 - 0.006097225869850511_f64 * t395 + 0.0010844166666666667_f64 * t107 * t1180;
    t1183
}
