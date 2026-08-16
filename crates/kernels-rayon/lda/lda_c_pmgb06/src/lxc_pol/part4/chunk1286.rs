//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1286/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1286(t1464: f64, t2599: f64, t10139: f64, t1080: f64, t493: f64, t1083: f64, t6507: f64, t5470: f64, t2386: f64, t9509: f64, t12592: f64, t16873: f64, t16874: f64, t16876: f64, t16878: f64, t16881: f64, t16883: f64, t16885: f64, t16886: f64, t16891: f64, t16894: f64, t16895: f64, t16899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16900 = t2599 * t1464;
    let t16904 = 2.0_f64 / 27.0_f64 * t493 * t10139 * t16900 * t1080;
    let t16905 = t6507 * t1083;
    let t16908 = 8.0_f64 / 81.0_f64 * t493 * t5470 * t16905;
    let t16910 = t9509 * t2386 * t1080;
    let t16913 = 88.0_f64 / 243.0_f64 * t493 * t12592 * t16910;
    let t16914 = t16873 + t16874 - t16876 - t16878 + t16881 + t16883 - t16885 - t16886 - t16891 + t16894 + t16895 + t16899 + t16904 + t16908 + t16913;
    (t16904, t16905, t16908, t16910, t16913, t16914)
}
