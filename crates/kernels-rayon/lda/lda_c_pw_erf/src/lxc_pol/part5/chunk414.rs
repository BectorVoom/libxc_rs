//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 414/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk414(t479: f64, t781: f64, t473: f64, t780: f64, t483: f64, t485: f64, t163: f64, t169: f64, t299: f64, t841: f64, t1235: f64, t1295: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1908 = t781 * t479;
    let t1910 = t473 * t780;
    let t1912 = t1910 * t483 * t485;
    let t1919 = t169 * t299 * t841 * t163;
    let t1922 = 4.0_f64 / 45.0_f64 * t1235;
    let t1923 = 4.0_f64 / 45.0_f64 * t1295;
    (t1908, t1910, t1912, t1919, t1922, t1923)
}
