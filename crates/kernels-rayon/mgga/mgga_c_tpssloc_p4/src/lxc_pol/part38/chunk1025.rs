//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1025/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1025(t12606: f64, t55: f64, t12677: f64, t12681: f64, t12684: f64, t12687: f64, t12695: f64, t12699: f64, t12702: f64, t1414: f64, t1420: f64, t2262: f64, t2275: f64, t2278: f64, t39: f64, t3982: f64, t3985: f64, t51: f64, t615: f64, t9311: f64) -> f64 {
    let t12705 = t55 * t12606;
    let t12708 = 220.0_f64 / 27.0_f64 * t2262 * t1414 - 40.0_f64 / 27.0_f64 * t615 * t3982 - 40.0_f64 / 9.0_f64 * t615 * t3985 - 5.0_f64 / 108.0_f64 * t39 * t12677 + 5.0_f64 / 9.0_f64 * t39 * t12681 + 5.0_f64 / 18.0_f64 * t39 * t12684 + 5.0_f64 / 6.0_f64 * t39 * t12687 - 20.0_f64 / 27.0_f64 * t1420 * t2275 + 20.0_f64 / 9.0_f64 * t1420 * t2278 + 5.0_f64 / 108.0_f64 * t51 * t12695 + 5.0_f64 / 9.0_f64 * t51 * t12699 + 5.0_f64 / 18.0_f64 * t51 * t12702 - 5.0_f64 / 6.0_f64 * t51 * t12705 + t9311;
    t12708
}
