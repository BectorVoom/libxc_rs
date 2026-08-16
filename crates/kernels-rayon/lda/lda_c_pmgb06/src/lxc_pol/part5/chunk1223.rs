//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1223/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1223(t19740: f64, t19741: f64, t19742: f64, t19746: f64, t19748: f64, t9457: f64, t9461: f64, t9467: f64, t9470: f64, t9478: f64, t9481: f64, t12225: f64, t12227: f64, t19935: f64, t19937: f64, t19939: f64, t19941: f64, t19943: f64, t19944: f64, t19945: f64, t19946: f64, t19947: f64, t9483: f64) -> (f64, f64) {
    let t21935 = -t19740 + t19741 + t19742 + t19746 + t19748 + 0.001515438175925926_f64 * t9457 + t9461 + t9467 + t9470 / 3.0_f64 + t9478 + t9481;
    let t21938 = 0.18233333333333332_f64 * t9483 - t12225 - 2.0_f64 / 3.0_f64 * t12227 - t19935 + t19937 - t19939 + t19941 + t19943 + t19944 + t19945 + t19946 + t19947;
    (t21935, t21938)
}
