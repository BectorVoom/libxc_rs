//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1205/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1205(t13000: f64, t15872: f64, t5083: f64, t1: f64, t5085: f64, t13043: f64, t5094: f64, t12991: f64, t15324: f64, t5499: f64, t6395: f64, t1972: f64, t5487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15879 = 4.0_f64 / 9.0_f64 * t5083 * t13000 * t15872;
    let t15880 = t5085 * t1;
    let t15883 = 16.0_f64 / 45.0_f64 * t13043 * t5094 * t15880;
    let t15886 = 16.0_f64 / 15.0_f64 * t13043 * t12991 * t15324;
    let t15887 = t5499 * t6395;
    let t15888 = 4.0_f64 / 27.0_f64 * t15887;
    let t15890 = 4.0_f64 / 45.0_f64 * t1972 * t5487;
    (t15879, t15880, t15883, t15886, t15888, t15890)
}
