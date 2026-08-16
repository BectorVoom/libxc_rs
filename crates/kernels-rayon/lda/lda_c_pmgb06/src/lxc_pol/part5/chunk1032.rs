//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1032/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1032(t19349: f64, t36: f64, t506: f64, t1464: f64, t337: f64, t7300: f64, t1476: f64, t1820: f64, t5974: f64, t2389: f64, t4865: f64, t1830: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19351 = t36 * t506 * t19349;
    let t19354 = t1464 * t7300 * t337;
    let t19356 = t36 * t1476 * t19354;
    let t19358 = t1820 * t5974;
    let t19360 = t36 * t1476 * t19358;
    let t19362 = t4865 * t2389;
    let t19364 = t1830 * t1476 * t19362;
    (t19351, t19354, t19356, t19358, t19360, t19362, t19364)
}
