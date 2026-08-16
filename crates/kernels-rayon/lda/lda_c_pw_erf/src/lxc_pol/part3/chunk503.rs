//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 503/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk503(t2076: f64, t551: f64, t595: f64, t822: f64, t331: f64, t803: f64, t1268: f64, t1967: f64, t1972: f64, t538: f64, t1976: f64, t1240: f64, t1241: f64, t1263: f64, t1264: f64, t1964: f64, t1969: f64, t1974: f64, t1978: f64, t2061: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2078 = 4.0_f64 / 15.0_f64 * t2076 * t551;
    let t2080 = 2.0_f64 / 15.0_f64 * t822 * t595;
    let t2087 = t331 * t803;
    let t2089 = t1268 * t1967;
    let t2092 = t538 * t1972;
    let t2095 = t538 * t1976;
    let t2098 = t1240 + 0.011997222222222222_f64 * t1241 + 0.011997222222222222_f64 * t1964 - 0.023994444444444443_f64 * t1969 + 0.07198333333333333_f64 * t1974 - 0.07198333333333333_f64 * t1978 + t1263 + 0.0044444444444444444_f64 * t1264 + 0.0044444444444444444_f64 * t2087 - 0.0022222222222222222_f64 * t25 * t2089 + 0.013333333333333334_f64 * t25 * t2092 - 0.013333333333333334_f64 * t2061 * t2095;
    (t2078, t2080, t2089, t2092, t2095, t2098)
}
