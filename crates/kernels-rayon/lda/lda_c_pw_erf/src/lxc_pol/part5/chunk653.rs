//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 653/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk653(t3783: f64, t798: f64, t519: f64, t3762: f64, t825: f64, t571: f64, t2192: f64, t3899: f64, t1318: f64, t2162: f64, t2167: f64, t3787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5339 = t3783 * t798;
    let t5340 = t519 * t5339;
    let t5342 = t3762 * t825;
    let t5343 = t571 * t5342;
    let t5363 = t3899 * t2192;
    let t5365 = 16.0_f64 / 45.0_f64 * t1318 * t5363;
    let t5371 = t3899 * t2162;
    let t5373 = 16.0_f64 / 45.0_f64 * t571 * t5371;
    let t5378 = t3787 * t2167;
    (t5339, t5340, t5342, t5343, t5363, t5365, t5371, t5373, t5378)
}
