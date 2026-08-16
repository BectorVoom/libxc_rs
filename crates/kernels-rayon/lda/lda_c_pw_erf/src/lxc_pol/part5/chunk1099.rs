//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1099/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1099(t443: f64, t7949: f64, t1870: f64, t5639: f64, t7970: f64, t14674: f64, t1832: f64, t1871: f64, t20301: f64, t20302: f64, t20303: f64, t2594: f64, t2610: f64, t411: f64, t5651: f64, t6121: f64, t756: f64, t7913: f64, t7918: f64, t8865: f64, t8869: f64, t8873: f64, t9083: f64) -> (f64, f64) {
    let t20440 = t7949 * t443;
    let t20493 = t1870 * t5639 * t7970;
    let t20507 = t8865 - t8869 + t8873 + 15.518295_f64 * t1870 * t1871 * t756 * t6121 + 5.172765_f64 * t1870 * t1871 * t7913 * t411 - 5.172765_f64 * t20493 + 103.4553_f64 * t1870 * t14674 * t7918 * t411 - 62.07318_f64 * t1870 * t5651 * t2594 * t1832 + 15.518295_f64 * t1870 * t1871 * t1832 * t2610 + t9083 - t20301 + t20302 - t20303;
    (t20440, t20507)
}
