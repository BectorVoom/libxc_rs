//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1076/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1076<F: Float>(t1848: F, t2601: F, t16238: F, t16241: F, t12232: F, t12234: F, t16249: F, t16254: F, t19935: F, t19937: F, t19939: F, t19941: F) -> (F, F, F, F, F, F, F, F) {
    let t19943 = t1848 * t2601 / F::cast_from(5.0_f64);
    let t19944 = t16238 / F::cast_from(15.0_f64);
    let t19945 = t16241 / F::cast_from(15.0_f64);
    let t19946 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t12232;
    let t19947 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t12234;
    let t19948 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t16249;
    let t19949 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t16254;
    let t19950 = -t19935 + t19937 - t19939 + t19941 + t19943 + t19944 + t19945 + t19946 + t19947 + t19948 + t19949;
    (t19943, t19944, t19945, t19946, t19947, t19948, t19949, t19950)
}
