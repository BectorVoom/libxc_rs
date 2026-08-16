//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1364/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1364(t1444: f64, t1979: f64, t1517: f64, t2642: f64, t1153: f64, t12381: f64, t12390: f64, t12392: f64, t1478: f64, t1483: f64, t1507: f64, t16534: f64, t16538: f64, t17540: f64, t17543: f64, t17548: f64, t17552: f64, t1995: f64, t2018: f64, t368: f64, t3810: f64, t3842: f64, t4193: f64, t5527: f64, t562: f64, t5938: f64, t86: f64) -> f64 {
    let t17556 = t1979 * t1444;
    let t17558 = t1517 * t17556 * t2642;
    let t17562 = -0.619125e-2_f64 * t562 * t16534 + 0.9286875e-2_f64 * t562 * t16538 - 0.619125e-2_f64 * t4193 * t1995 - 0.123825e-1_f64 * t1507 * t5527 + 0.1857375e-1_f64 * t5938 * t1478 - 0.123825e-1_f64 * t5938 * t1483 + 0.9286875e-2_f64 * t2018 * t3810 - 0.619125e-2_f64 * t2018 * t3842 + 0.35374814814814814814e-1_f64 * t12381 - 0.35374814814814814814e-1_f64 * t17540 - 0.26531111111111111111e-1_f64 * t1153 * t17543 - 0.53062222222222222222e-1_f64 * t1153 * t17548 - 0.26531111111111111111e-1_f64 * t12390 - 0.39796666666666666666e-1_f64 * t86 * t368 * t17552 + 0.53062222222222222222e-1_f64 * t1153 * t17558 - 0.17687407407407407407e-1_f64 * t12392;
    t17562
}
