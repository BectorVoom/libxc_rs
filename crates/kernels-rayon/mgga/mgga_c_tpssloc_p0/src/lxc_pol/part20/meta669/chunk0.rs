//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2515/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2515(t11243: f64, t3271: f64, t4756: f64, t1102: f64, t14758: f64, t3270: f64, t3287: f64, t51000: f64, t51004: f64, t51007: f64, t51010: f64, t51012: f64, t51014: f64, t51016: f64, t51018: f64, t51021: f64) -> (f64, f64, f64, f64) {
    let t51024 = t11243 * t4756 * t3271;
    let t51027 = t3270 * t14758 * t1102;
    let t51030 = t3287 * t14758 * t1102;
    let t51032 = 0.543465e1_f64 * t51000 + 0.10064166666666666667e1_f64 * t51004 - 0.485484375e1_f64 * t51007 + 0.6189328125e-1_f64 * t51010 - 0.3883875e1_f64 * t51012 - 0.1294625e1_f64 * t51014 + 0.247573125e0_f64 * t51016 + 0.82524375e-1_f64 * t51018 + 0.58258125e1_f64 * t51021 - 0.1237865625e0_f64 * t51024 - 0.3883875e1_f64 * t51027 + 0.247573125e0_f64 * t51030;
    (t51024, t51027, t51030, t51032)
}
