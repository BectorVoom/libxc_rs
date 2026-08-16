//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1101/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1101(t133: f64, t20430: f64, t20427: f64, t14585: f64, t14641: f64, t14652: f64, t19773: f64, t19775: f64, t19782: f64, t20340: f64, t20341: f64, t20342: f64, t20345: f64, t20353: f64) -> f64 {
    let t20516 = t133 * t20430;
    let t20518 = t133 * t20427;
    let t20525 = -2.2990066666666666_f64 * t14585 + 6.89702_f64 * t20516 + 0.5747516666666667_f64 * t20518 - 1.724255_f64 * t133 * t20345 - t14641 + t14652 + t20340 - t20341 - t20342 - 2.2990066666666666_f64 * t19773 + 6.89702_f64 * t19775 + 1.724255_f64 * t19782 + t20353;
    t20525
}
