//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 797/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk797(t2871: f64, t5336: f64, t493: f64, t5301: f64, t5304: f64, t5307: f64, t5309: f64, t5311: f64, t5315: f64, t5317: f64, t5321: f64, t5324: f64, t5325: f64, t5328: f64, t5330: f64, t5332: f64, t5335: f64) -> (f64, f64, f64) {
    let t5337 = t2871 * t5336;
    let t5339 = 2.0_f64 / 45.0_f64 * t493 * t5337;
    let t5340 = t5301 - t5304 + t5307 + t5309 + t5311 + t5315 + t5317 + t5321 - t5324 - t5325 - t5328 - t5330 + t5332 + t5335 + t5339;
    (t5337, t5339, t5340)
}
