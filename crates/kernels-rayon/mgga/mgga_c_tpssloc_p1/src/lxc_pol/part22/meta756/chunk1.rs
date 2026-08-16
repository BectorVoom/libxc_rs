//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2540/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2540(t50846: f64, t50854: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64) -> f64 {
    let t71440 = -0.11182407407407407407e0_f64 * t71146 + 0.301925e0_f64 * t71150 - 0.60385e0_f64 * t71152 - 0.10064166666666666667e0_f64 * t71154 + 0.40256666666666666667e0_f64 * t71156 - 0.73586666666666666667e0_f64 * t50846 + t50854 + 0.10064166666666666667e1_f64 * t71160 - 0.89459259259259259259e0_f64 * t71166 + 0.543465e1_f64 * t71170 + 0.72462e1_f64 * t71174 + 0.60385e0_f64 * t71179;
    t71440
}
