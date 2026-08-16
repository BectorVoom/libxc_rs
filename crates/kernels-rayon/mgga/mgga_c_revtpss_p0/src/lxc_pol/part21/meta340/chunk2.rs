//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1662/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1662(t11467: f64, t3014: f64, t11132: f64, t11337: f64, t11158: f64, t11162: f64, t11167: f64, t11316: f64, t11319: f64, t11322: f64, t11326: f64, t11329: f64, t11332: f64, t11339: f64, t11343: f64, t11346: f64) -> (f64, f64, f64, f64) {
    let t11468 = t11467 * t3014;
    let t11479 = 0.93932222222222222223e0_f64 * t11132;
    let t11480 = 0.36793333333333333333e0_f64 * t11337;
    let t11485 = 0.16504875e0_f64 * t11316 - 0.82785e-1_f64 * t11319 + 0.49671e0_f64 * t11322 + 0.181155e1_f64 * t11167 - 0.60384999999999999999e0_f64 * t11158 - 0.33114e0_f64 * t11326 + 0.16557e0_f64 * t11329 - 0.49671e0_f64 * t11332 - t11479 - t11480 + 0.5519e-1_f64 * t11339 - 0.36793333333333333333e-1_f64 * t11343 - 0.82785e-1_f64 * t11346 - 0.181155e1_f64 * t11162;
    (t11468, t11479, t11480, t11485)
}
