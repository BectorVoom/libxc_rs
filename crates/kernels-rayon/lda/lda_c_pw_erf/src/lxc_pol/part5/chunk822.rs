//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 822/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk822(t4185: f64, t4198: f64, t4201: f64, t4206: f64, t4209: f64, t4544: f64, t4547: f64, t4719: f64, t7256: f64, t7530: f64, t7531: f64, t7532: f64, t7534: f64, t7536: f64, t7538: f64, t7540: f64, t7541: f64) -> f64 {
    let t7544 = 0.21642082724729686_f64 * t4544 + 0.03354522822333102_f64 * t4547 - t4185 + t4198 + t4201 + t4206 - t4209 + t7530 - t7531 - t7532 - t7534 + t7536 + t7538 + t7540 - t7541 + 4.0_f64 * t4719 + 4.0_f64 * t7256;
    t7544
}
