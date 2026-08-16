//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1047/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1047(t12456: f64, t432: f64, t4817: f64, t835: f64, t9266: f64, t1977: f64, t3223: f64, t11862: f64, t160: f64, t1983: f64, t2983: f64, t5068: f64, t5090: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12457 = 4.0_f64 / 405.0_f64 * t12456;
    let t12459 = t432 * t4817 / 5.0_f64;
    let t12460 = t9266 * t835;
    let t12461 = 2.0_f64 / 135.0_f64 * t12460;
    let t12462 = t3223 * t1977;
    let t12463 = 2.0_f64 / 135.0_f64 * t12462;
    let t12465 = t160 * t11862 * t1983;
    let t12466 = 32.0_f64 / 135.0_f64 * t12465;
    let t12469 = 2.0_f64 / 15.0_f64 * t5068 * t5090 * t2983;
    (t12457, t12459, t12461, t12463, t12466, t12469)
}
