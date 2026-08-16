//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 862/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk862(t3706: f64, t3917: f64, t3919: f64, t3923: f64, t3929: f64, t3935: f64, t3938: f64, t5806: f64, t7519: f64, t7524: f64, t7530: f64, t7531: f64, t7532: f64, t7534: f64, t7536: f64, t7538: f64, t7540: f64, t7541: f64) -> f64 {
    let t8031 = t7519 - t7524 + 2.0_f64 / 45.0_f64 * t5806 - t3706 + t7530 - t7531 - t7532 - t7534 + t7536 + t7538 + t7540 - t7541 + t3917 + t3919 + t3923 + t3929 + t3935 - t3938;
    t8031
}
