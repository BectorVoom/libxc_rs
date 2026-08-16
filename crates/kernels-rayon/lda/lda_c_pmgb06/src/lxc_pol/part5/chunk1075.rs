//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1075/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1075(t132: f64, t137: f64, t19820: f64, t19862: f64, t19897: f64, t19929: f64, t465: f64, t1423: f64, t7696: f64, t486: f64, t7748: f64, t6449: f64, t831: f64) -> (f64, f64, f64, f64) {
    let t19935 = t132 * t137 * t465 * (t19820 + t19862 + t19897 + t19929) / 30.0_f64;
    let t19936 = t1423 * t7696;
    let t19937 = 2.0_f64 / 15.0_f64 * t19936;
    let t19939 = t486 * t7748 / 10.0_f64;
    let t19941 = t831 * t6449 / 5.0_f64;
    (t19935, t19937, t19939, t19941)
}
