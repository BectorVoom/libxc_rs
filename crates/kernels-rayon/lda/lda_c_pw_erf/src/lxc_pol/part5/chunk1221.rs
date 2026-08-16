//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1221/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1221(t1318: f64, t1319: f64, t1446: f64, t15931: f64, t2146: f64, t2389: f64, t2419: f64, t34: f64, t3416: f64, t4753: f64, t4758: f64, t4763: f64, t5334: f64, t6275: f64, t6397: f64, t6479: f64, t6483: f64, t6665: f64, t743: f64, t7734: f64, t7803: f64, t7822: f64, t811: f64) -> f64 {
    let t22048 = 8.0_f64 / 15.0_f64 * t4753 * t7734 + 8.0_f64 / 15.0_f64 * t3416 * t7734 + 8.0_f64 / 15.0_f64 * t1318 * t1319 * t6665 * t811 - 8.0_f64 / 15.0_f64 * t1318 * t4758 * t2419 * t34 - 4.0_f64 / 15.0_f64 * t1446 * t7803 - 16.0_f64 / 15.0_f64 * t4763 * t6479 - 32.0_f64 / 15.0_f64 * t4763 * t6483 + 8.0_f64 / 15.0_f64 * t4753 * t7822 + 8.0_f64 / 15.0_f64 * t3416 * t7822 + 8.0_f64 / 15.0_f64 * t1318 * t1319 * t15931 * t743 - 16.0_f64 / 15.0_f64 * t1318 * t4758 * t6275 * t34 - 8.0_f64 / 15.0_f64 * t5334 * t2389 - 8.0_f64 / 15.0_f64 * t2146 * t6397;
    t22048
}
