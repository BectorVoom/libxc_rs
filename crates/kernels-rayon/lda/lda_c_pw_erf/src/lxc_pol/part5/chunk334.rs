//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 334/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk334(t191: f64, t299: f64, t187: f64, t190: f64, t331: f64, t539: f64, t176: f64, t177: f64) -> (f64, f64, f64, f64, f64) {
    let t1260 = t299 * t191;
    let t1263 = 0.011111111111111112_f64 * t190 * t1260 * t187;
    let t1264 = t331 * t539;
    let t1267 = 1.0_f64 / t177 / t176;
    let t1268 = t191 * t1267;
    (t1260, t1263, t1264, t1267, t1268)
}
