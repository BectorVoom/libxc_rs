//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2931/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2931(t10160: f64, t1052: f64, t13736: f64, t13743: f64, t14526: f64, t14545: f64, t14549: f64, t14555: f64, t14659: f64, t1603: f64, t17575: f64, t17583: f64, t17588: f64, t18062: f64, t3166: f64, t3169: f64, t3174: f64, t3176: f64, t3206: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t4694: f64, t5848: f64, t5943: f64, t5944: f64) -> f64 {
    let t61048 = 2.0_f64 * t1052 * t3174 * t3206 * t5943 + 2.0_f64 * t14526 * t1603 * t388 + t3166 * t388 * t5848 - 2.0_f64 * t10160 * t5944 - 12.0_f64 * t13736 * t4557 - 12.0_f64 * t13736 * t4660 + 8.0_f64 * t13743 * t4557 + 8.0_f64 * t13743 * t4660 + 8.0_f64 * t14545 * t4665 + 4.0_f64 * t14549 * t4557 + 4.0_f64 * t14549 * t4660 - 4.0_f64 * t14555 * t4694 - 2.0_f64 * t14659 * t4660 + 2.0_f64 * t17575 * t3176 + 8.0_f64 * t17583 * t3169 + 4.0_f64 * t17588 * t3176 + 4.0_f64 * t18062 * t3169;
    t61048
}
