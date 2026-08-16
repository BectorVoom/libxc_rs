//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1211/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1211(t1472: f64, t5322: f64, t2163: f64, t3727: f64, t3416: f64, t5317: f64, t2171: f64, t3896: f64, t10502: f64, t799: f64, t10675: f64, t10678: f64, t10680: f64, t10685: f64, t14271: f64, t14275: f64, t14278: f64, t14283: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14285 = 8.0_f64 / 5.0_f64 * t1472 * t5322;
    let t14287 = 4.0_f64 / 5.0_f64 * t3727 * t2163;
    let t14289 = 8.0_f64 / 5.0_f64 * t3416 * t5317;
    let t14291 = 32.0_f64 / 81.0_f64 * t2171 * t3896;
    let t14293 = 4.0_f64 / 45.0_f64 * t10502 * t799;
    let t14296 = t14271 - t14275 - t14278 + t14283 + t14285 + t14287 - t14289 + t14291 + t14293 + t10675 + 0.10821041362364843_f64 * t10678 + 0.6492624817418906_f64 * t10680 + t10685;
    (t14285, t14287, t14289, t14291, t14293, t14296)
}
