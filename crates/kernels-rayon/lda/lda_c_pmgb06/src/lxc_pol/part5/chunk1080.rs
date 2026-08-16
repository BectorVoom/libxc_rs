//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1080/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1080(t12456: f64, t12465: f64, t16380: f64, t16383: f64, t477: f64, t7458: f64, t5077: f64, t5094: f64, t12991: f64, t19618: f64, t12995: f64, t5083: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19985 = 4.0_f64 / 135.0_f64 * t12456;
    let t19986 = 4.0_f64 / 135.0_f64 * t12465;
    let t19987 = 4.0_f64 / 45.0_f64 * t16380;
    let t19988 = 2.0_f64 / 45.0_f64 * t16383;
    let t19989 = t7458 * t477;
    let t19992 = 2.0_f64 / 15.0_f64 * t5077 * t5094 * t19989;
    let t19995 = 2.0_f64 / 5.0_f64 * t5077 * t12991 * t19618;
    let t19998 = 2.0_f64 / 3.0_f64 * t5083 * t12995 * t19618;
    (t19985, t19986, t19987, t19988, t19989, t19992, t19995, t19998)
}
