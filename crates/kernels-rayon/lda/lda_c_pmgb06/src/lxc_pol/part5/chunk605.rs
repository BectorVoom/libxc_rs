//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 605/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk605(t4298: f64, t4299: f64, t1759: f64, t707: f64, t1763: f64, t1183: f64, t301: f64, t398: f64, t297: f64, t122: f64, t4182: f64, t302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4301 = 4.569219094474146e-06_f64 * t4298 * t4299;
    let t4304 = t707 * t1759;
    let t4307 = 0.05987117005127304_f64 * t707 * t1763;
    let t4317 = t398 * t1183 * t301;
    let t4318 = t297 * t4317;
    let t4320 = t122 * t4182;
    let t4322 = 0.19513566535229734_f64 * t4320 * t302;
    (t4301, t4304, t4307, t4317, t4318, t4320, t4322)
}
