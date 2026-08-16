//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 790/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk790(t1915: f64, t4852: f64, t176: f64, t3238: f64, t1821: f64, t1919: f64, t4880: f64, t3248: f64, t4885: f64, t4866: f64, t1924: f64, t2979: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5458 = t1915 * t4852;
    let t5463 = t3238 * t176;
    let t5464 = t5463 * t1821;
    let t5467 = t1919 * t4880;
    let t5470 = t3248 * t176;
    let t5471 = t5470 * t4885;
    let t5474 = t1919 * t4866;
    let t5477 = t2979 * t1924;
    (t5458, t5463, t5464, t5467, t5470, t5471, t5474, t5477)
}
