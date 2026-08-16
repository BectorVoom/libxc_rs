//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 722/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk722(t3664: f64, t1: f64, t1904: f64, t3: f64, t604: f64, t1635: f64, t1926: f64, t3439: f64, t4514: f64, t4520: f64, t4525: f64, t4526: f64, t4527: f64, t4528: f64, t4529: f64, t4530: f64, t4531: f64, t4532: f64, t4533: f64, t4534: f64) -> (f64, f64, f64) {
    let t4535 = 8.0_f64 / 135.0_f64 * t3664;
    let t4537 = t1904 * t1 * t3;
    let t4539 = 0.21642082724729686_f64 * t4537 * t604;
    let t4540 = t1926 * t1635;
    let t4542 = t4514 + t3439 + t4520 - t4525 + t4526 + t4527 - t4528 - t4529 + t4530 - t4531 - t4532 - t4533 - t4534 + t4535 + t4539 + 0.21642082724729686_f64 * t4540;
    (t4535, t4537, t4542)
}
