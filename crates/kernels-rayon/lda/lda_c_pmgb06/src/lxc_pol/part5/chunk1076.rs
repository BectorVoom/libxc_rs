//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1076/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1076(t1848: f64, t2601: f64, t16238: f64, t16241: f64, t12232: f64, t12234: f64, t16249: f64, t16254: f64, t19935: f64, t19937: f64, t19939: f64, t19941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19943 = t1848 * t2601 / 5.0_f64;
    let t19944 = t16238 / 15.0_f64;
    let t19945 = t16241 / 15.0_f64;
    let t19946 = 4.0_f64 / 135.0_f64 * t12232;
    let t19947 = 4.0_f64 / 135.0_f64 * t12234;
    let t19948 = 2.0_f64 / 15.0_f64 * t16249;
    let t19949 = 4.0_f64 / 15.0_f64 * t16254;
    let t19950 = -t19935 + t19937 - t19939 + t19941 + t19943 + t19944 + t19945 + t19946 + t19947 + t19948 + t19949;
    (t19943, t19944, t19945, t19946, t19947, t19948, t19949, t19950)
}
