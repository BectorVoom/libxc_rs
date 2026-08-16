//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1412/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1412(t22411: f64, t22413: f64, t22415: f64, t22417: f64, t22420: f64, t22423: f64, t22425: f64, t22428: f64, t22431: f64, t22433: f64, t22437: f64, t22471: f64, t22632: f64, t22634: f64, t22638: f64, t22641: f64, t22643: f64, t22645: f64, t22647: f64, t22650: f64, t22653: f64, t22655: f64) -> (f64, f64) {
    let t23321 = -0.20833333333333333333e-1_f64 * t22411 + 0.26979166666666666666e-1_f64 * t22413 - 0.20234375e-1_f64 * t22415 - 0.125e0_f64 * t22417 + 0.61111111111111111111e0_f64 * t22420 - 0.45564814814814814815e0_f64 * t22423 + 0.20234375e-1_f64 * t22425 + 0.34173611111111111111e0_f64 * t22428 - 0.4046875e-1_f64 * t22431 - 0.20833333333333333333e-1_f64 * t22433 + 0.625e-1_f64 * t22437;
    let t23346 = -0.44965277777777777777e-2_f64 * t22471 + 0.9375e-1_f64 * t22632 + 0.1875e0_f64 * t22634 - 0.101171875e-1_f64 * t22638 + 0.13489583333333333333e-1_f64 * t22641 + 0.10791666666666666667e0_f64 * t22643 - 0.1875e0_f64 * t22645 + 0.5e0_f64 * t22647 + 0.60703125e-1_f64 * t22650 - 0.10791666666666666667e0_f64 * t22653 + 0.101171875e-1_f64 * t22655;
    (t23321, t23346)
}
