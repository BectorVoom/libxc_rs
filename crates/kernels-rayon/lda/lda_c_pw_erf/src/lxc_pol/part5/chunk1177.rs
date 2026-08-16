//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1177/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1177(t21444: f64, t15761: f64, t786: f64, t2123: f64, t2402: f64, t21423: f64, t21426: f64, t21427: f64, t21428: f64, t21430: f64, t21431: f64, t21432: f64, t21436: f64, t21438: f64, t21442: f64) -> (f64, f64, f64, f64) {
    let t21445 = 8.0_f64 / 45.0_f64 * t21444;
    let t21447 = 4.0_f64 / 5.0_f64 * t15761 * t786;
    let t21448 = t2402 * t2123;
    let t21449 = 8.0_f64 / 15.0_f64 * t21448;
    let t21450 = t21423 + t21426 - t21427 + t21428 - t21430 - t21431 - t21432 + t21436 - t21438 + t21442 + t21445 + t21447 - t21449;
    (t21445, t21447, t21449, t21450)
}
