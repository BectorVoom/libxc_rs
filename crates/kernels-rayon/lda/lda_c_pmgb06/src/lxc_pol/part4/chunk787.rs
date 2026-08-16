//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 787/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk787(t332: f64, t5231: f64, t1385: f64, t439: f64, t1420: f64, t1908: f64, t1907: f64, t2948: f64, t5196: f64, t5200: f64, t5205: f64, t5207: f64, t5209: f64, t5213: f64, t5215: f64, t5217: f64, t5219: f64, t5222: f64, t5224: f64, t5228: f64, t5230: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5232 = t5231 * t332;
    let t5233 = t1385 * t5232;
    let t5235 = 2.0_f64 / 45.0_f64 * t439 * t5233;
    let t5237 = 2.0_f64 / 45.0_f64 * t1420 * t1908;
    let t5238 = t2948 * t1907;
    let t5240 = 2.0_f64 / 45.0_f64 * t439 * t5238;
    let t5241 = t5196 + t5200 + t5205 + t5207 + t5209 - t5213 + t5215 + t5217 + t5219 + t5222 - t5224 - t5228 - t5230 - t5235 - t5237 - t5240;
    (t5232, t5233, t5235, t5237, t5238, t5240, t5241)
}
