//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 610/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk610(t11: f64, t3501: f64, t3412: f64, t503: f64, t25: f64, t3472: f64, t3473: f64, t3478: f64, t3483: f64, t3487: f64, t3490: f64, t3493: f64, t3496: f64, t3499: f64) -> (f64, f64, f64, f64) {
    let t3502 = t11 * t3501;
    let t3504 = t503 * t3412;
    let t3505 = t11 * t3504;
    let t3507 = -t3472 - 0.02666666666666667_f64 * t3473 + 0.013333333333333334_f64 * t25 * t3478 - 0.006666666666666667_f64 * t25 * t3483 - 0.04_f64 * t25 * t3487 + 0.04_f64 * t25 * t3490 - 0.07198333333333333_f64 * t3493 + 0.14396666666666666_f64 * t3496 - 0.07198333333333333_f64 * t3499 - 0.21595_f64 * t3502 + 0.21595_f64 * t3505;
    (t3502, t3504, t3505, t3507)
}
