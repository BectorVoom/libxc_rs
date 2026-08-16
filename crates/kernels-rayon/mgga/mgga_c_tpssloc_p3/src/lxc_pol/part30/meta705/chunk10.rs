//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2318/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2318(t28719: f64, t3216: f64, t100489: f64, t1068: f64, t1070: f64, t1637: f64, t18169: f64, t193: f64, t23738: f64, t23742: f64, t25840: f64, t25845: f64, t336: f64, t4696: f64, t4700: f64, t5946: f64, t5950: f64, t6822: f64, t83472: f64, t83479: f64, t89698: f64, t89702: f64, t99104: f64, t99143: f64, t99172: f64, t99202: f64, t99238: f64, t99271: f64, t99313: f64, t99353: f64, t99390: f64, t99422: f64, t99450: f64, t99866: f64, t99894: f64, t99930: f64, t99959: f64) -> f64 {
    let t100497 = t28719 * t3216;
    let t100528 = t193 * t336 * (t99104 + t99143 + t99172 + t99202 + t99238 + t99271 + t99313 + t99353 + t99390 + t99422 + t99450 + t99866 + t99894 + t99930 + t99959 + t100489) * t1070 - t4700 * t100497 * t1068 - 2.0_f64 * t4700 * t89698 * t1637 + 4.0_f64 * t4700 * t89702 * t25845 - 2.0_f64 * t4700 * t25840 * t4696 + 2.0_f64 * t4700 * t83472 * t5950 - 6.0_f64 * t4700 * t83479 * t5950 * t1068 + 4.0_f64 * t4700 * t23742 * t1637 * t4696 - t4700 * t23738 * t5946 + 2.0_f64 * t4700 * t23742 * t5946 * t1068 - t4700 * t6822 * t18169;
    t100528
}
