//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2952/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2952(t2986: f64, t4514: f64, t48019: f64, t48046: f64, t10186: f64, t10259: f64, t17742: f64, t17745: f64, t17749: f64, t17794: f64, t17801: f64, t17817: f64, t25608: f64, t3014: f64, t343: f64, t4510: f64, t4518: f64, t4531: f64, t4546: f64, t5836: f64, t59719: f64, t59746: f64, t884: f64, t973: f64) -> f64 {
    let t61489 = t2986 * t48019 * t4514;
    let t61495 = t2986 * t48046 * t4514;
    let t61523 = 0.12345679012345679012e-3_f64 * t61489 + 0.16666666666666666666e-2_f64 * t2986 * t4518 * t59746 - 0.37037037037037037036e-3_f64 * t61495 + 0.55555555555555555554e-3_f64 * t2986 * t10259 * t17817 - 0.27777777777777777777e-3_f64 * t2986 * t10259 * t17794 + 0.14814814814814814814e-2_f64 * t10186 * t17801 + 0.74074074074074074072e-3_f64 * t2986 * t4510 * t59719 - 0.59259259259259259257e-2_f64 * t10186 * t17742 + 0.39506172839506172838e-2_f64 * t10186 * t17745 + 0.29629629629629629628e-2_f64 * t10186 * t17749 - 0.11111111111111111111e-2_f64 * t2986 * t4531 * t25608 * t884 - 0.83333333333333333332e-3_f64 * t973 * t4546 * t5836 * t3014 * t343;
    t61523
}
