//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1058/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1058(t1830: f64, t2186: f64, t1180: f64, t776: f64, t360: f64, t5793: f64, t947: f64, t2060: f64, t5796: f64, t5799: f64, t5802: f64, t2221: f64, t410: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11390 = t2186 * t1830;
    let t11392 = t1180 * t776;
    let t11393 = t360 * t11392;
    let t11395 = t5793 * t947;
    let t11398 = t5796 * t2060;
    let t11400 = t5799 * t947;
    let t11402 = t5802 * t2060;
    let t11404 = t410 * t2221;
    (t11390, t11392, t11393, t11395, t11398, t11400, t11402, t11404)
}
