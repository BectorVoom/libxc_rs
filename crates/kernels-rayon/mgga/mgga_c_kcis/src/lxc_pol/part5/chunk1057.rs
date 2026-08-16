//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1057/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1057(t1409: f64, t4023: f64, t1441: f64, t1650: f64, t11951: f64, t12048: f64, t167: f64, t1444: f64, t2622: f64, t1445: f64, t5654: f64, t12065: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17088 = t4023 * t1409;
    let t17096 = t1441 * t1650;
    let t17098 = t11951 * t1650;
    let t17100 = t12048 * t167;
    let t17102 = t2622 * t1444;
    let t17103 = t17102 * t167;
    let t17137 = 0.47822877300252710492e-1_f64 * t1445 * t5654;
    let t17143 = 0.62154466893555682512e-3_f64 * t12065 * t5654;
    (t17088, t17096, t17098, t17100, t17103, t17137, t17143)
}
