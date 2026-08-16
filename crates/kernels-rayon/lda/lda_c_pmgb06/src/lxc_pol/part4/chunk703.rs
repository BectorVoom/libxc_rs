//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 703/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk703(t4298: f64, t4299: f64, t1755: f64, t707: f64, t1759: f64, t1763: f64, t1166: f64, t301: f64, t413: f64, t297: f64, t1183: f64, t398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4301 = 4.569219094474146e-06_f64 * t4298 * t4299;
    let t4302 = t707 * t1755;
    let t4304 = t707 * t1759;
    let t4307 = 0.05987117005127304_f64 * t707 * t1763;
    let t4313 = t1166 * t413 * t301;
    let t4314 = t297 * t4313;
    let t4317 = t398 * t1183 * t301;
    (t4301, t4302, t4304, t4307, t4313, t4314, t4317)
}
