//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 638/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk638(t132: f64, t3055: f64, t1540: f64, t464: f64, t1392: f64, t432: f64, t1396: f64, t435: f64, t1490: f64, t489: f64, t161: f64, t1541: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3056 = t132 * t3055;
    let t3058 = t1540 * t464;
    let t3064 = t432 * t1392;
    let t3068 = t435 * t1396;
    let t3069 = t132 * t3068;
    let t3073 = t489 * t1490;
    let t3074 = t161 * t3073;
    let t3076 = t435 * t1541;
    (t3056, t3058, t3064, t3068, t3069, t3073, t3074, t3076)
}
