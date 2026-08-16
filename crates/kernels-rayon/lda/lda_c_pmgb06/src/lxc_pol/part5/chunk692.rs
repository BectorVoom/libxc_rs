//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 692/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk692(t4779: f64, t805: f64, t439: f64, t1962: f64, t2011: f64, t2010: f64, t6265: f64, t6267: f64, t6270: f64, t6272: f64, t6274: f64, t6277: f64, t6279: f64, t6281: f64, t6284: f64, t6289: f64, t6291: f64, t6294: f64, t6296: f64, t6299: f64) -> (f64, f64, f64, f64, f64) {
    let t6300 = t4779 * t805;
    let t6302 = 2.0_f64 / 45.0_f64 * t439 * t6300;
    let t6303 = t1962 * t2011;
    let t6305 = 4.0_f64 / 45.0_f64 * t2010 * t6303;
    let t6306 = t6265 + t6267 - t6270 + t6272 + t6274 + t6277 + t6279 + t6281 + t6284 - t6289 - t6291 - t6294 - t6296 - t6299 + t6302 + t6305;
    (t6300, t6302, t6303, t6305, t6306)
}
