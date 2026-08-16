//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2282/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2282(t17632: f64, t17637: f64, t17643: f64, t17688: f64, t17718: f64, t17976: f64, t17980: f64, t23541: f64, t25580: f64, t4585: f64, t4590: f64, t6765: f64, t82885: f64, t83065: f64, t88281: f64) -> f64 {
    let t99535 = -t6765 * t17976 / 576.0_f64 - 5.0_f64 / 1152.0_f64 * t6765 * t17688 - t6765 * t17637 / 1152.0_f64 - t25580 * t4585 / 576.0_f64 + 5.0_f64 / 3456.0_f64 * t25580 * t4590 + t88281 + t82885 / 1296.0_f64 - t23541 * t17718 / 1536.0_f64 + 5.0_f64 / 6912.0_f64 * t6765 * t17643 - t23541 * t17632 / 768.0_f64 + t83065 * t17980 / 1536.0_f64;
    t99535
}
