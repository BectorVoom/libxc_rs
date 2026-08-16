//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 767/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk767(t183: f64, t6716: f64, t188: f64, t5215: f64, t5217: f64, t5219: f64, t5222: f64, t5304: f64, t5328: f64, t5330: f64, t5342: f64, t5349: f64, t6570: f64, t6574: f64, t6576: f64, t6578: f64, t6579: f64) -> (f64, f64) {
    let t7209 = t6716 * t183;
    let t7212 = t6570 + t5215 + t5217 + t5219 + t5222 - t5304 - t6574 - t6576 - t6578 + 4.0_f64 / 3.0_f64 * t7209 * t188 - t6579 - t5328 - t5330 - t5342 - t5349;
    (t7209, t7212)
}
