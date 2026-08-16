//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1270/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1270(t3469: f64, t40358: f64, t10610: f64, t3465: f64, t40285: f64, t38334: f64, t38339: f64, t38356: f64, t38359: f64, t39122: f64, t39127: f64, t39129: f64, t39130: f64, t39131: f64, t39134: f64, t42330: f64, t42334: f64, t42339: f64, t42344: f64) -> (f64, f64, f64) {
    let t42346 = t40358 * t3469 / 4.0_f64;
    let t42349 = 3.0_f64 / 2.0_f64 * t10610 * t3465 * t40285;
    let t42350 = -t42330 + t42334 + t39122 - 0.30487649791575028312e-3_f64 * t38334 + t39127 + 0.325201597776800302e-2_f64 * t38339 - t39129 + t39130 - t39131 - t42339 - 0.76845137554657911361e-2_f64 * t38356 + 0.12195059916630011325e-2_f64 * t38359 + t39134 + t42344 - t42346 + t42349;
    (t42346, t42349, t42350)
}
