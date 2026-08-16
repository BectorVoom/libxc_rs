//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1100/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1100(t10313: f64, t1967: f64, t197: f64, t519: f64, t11746: f64, t5256: f64, t518: f64, t5210: f64, t1322: f64, t12299: f64, t1329: f64, t10474: f64, t2007: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12869 = t519 * t10313 * t197 * t1967;
    let t12870 = 8.0_f64 / 81.0_f64 * t12869;
    let t12873 = 16.0_f64 / 3.0_f64 * t519 * t5256 * t11746;
    let t12874 = t5210 * t518;
    let t12876 = 16.0_f64 / 15.0_f64 * t12874 * t1322;
    let t12878 = 16.0_f64 / 15.0_f64 * t12299 * t1329;
    let t12880 = 8.0_f64 / 15.0_f64 * t10474 * t2007;
    (t12870, t12873, t12874, t12876, t12878, t12880)
}
