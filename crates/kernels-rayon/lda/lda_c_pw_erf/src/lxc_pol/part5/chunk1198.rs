//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1198/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1198(t17664: f64, t595: f64, t7676: f64, t544: f64, t7661: f64, t184: f64, t202: f64, t7674: f64, t551: f64, t17684: f64, t17687: f64, t17690: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21717 = 32.0_f64 / 45.0_f64 * t17664;
    let t21719 = 2.0_f64 / 15.0_f64 * t7676 * t595;
    let t21721 = 2.0_f64 / 15.0_f64 * t7661 * t544;
    let t21723 = t202 * t7674 * t184;
    let t21725 = 4.0_f64 / 15.0_f64 * t21723 * t551;
    let t21726 = 16.0_f64 / 45.0_f64 * t17684;
    let t21727 = 32.0_f64 / 45.0_f64 * t17687;
    let t21728 = 64.0_f64 / 45.0_f64 * t17690;
    (t21717, t21719, t21721, t21725, t21726, t21727, t21728)
}
