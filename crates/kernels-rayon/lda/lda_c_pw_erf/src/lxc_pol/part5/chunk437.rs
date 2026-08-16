//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 437/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk437(t1954: f64, t589: f64, t1346: f64, t1347: f64, t1366: f64, t1367: f64, t1941: f64, t1946: f64, t1951: f64, t1956: f64, t2053: f64, t2055: f64, t2058: f64, t2061: f64, t25: f64) -> (f64, f64) {
    let t2062 = t589 * t1954;
    let t2065 = t1346 + 0.011997222222222222_f64 * t1347 + 0.011997222222222222_f64 * t1941 - 0.023994444444444443_f64 * t1946 + 0.07198333333333333_f64 * t1951 + 0.07198333333333333_f64 * t1956 + t1366 + 0.0044444444444444444_f64 * t1367 + 0.0044444444444444444_f64 * t2053 - 0.0022222222222222222_f64 * t25 * t2055 + 0.013333333333333334_f64 * t25 * t2058 + 0.013333333333333334_f64 * t2061 * t2062;
    (t2062, t2065)
}
