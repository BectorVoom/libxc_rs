//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 930/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk930(t835: f64, t9266: f64, t1977: f64, t3223: f64, t11862: f64, t160: f64, t1983: f64, t27: f64, t34: f64, t1435: f64, t5075: f64, t1438: f64, t1593: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12460 = t9266 * t835;
    let t12461 = 2.0_f64 / 135.0_f64 * t12460;
    let t12462 = t3223 * t1977;
    let t12463 = 2.0_f64 / 135.0_f64 * t12462;
    let t12465 = t160 * t11862 * t1983;
    let t12514 = t27 * t34;
    let t12516 = t5075 * t12514 * t1435;
    let t12519 = t1593 * t1438;
    (t12461, t12463, t12465, t12514, t12516, t12519)
}
