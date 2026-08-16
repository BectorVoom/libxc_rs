//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1300/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1300(t2169: f64, t8119: f64, t1851: f64, t8927: f64, t34401: f64, t576: f64, t112: f64, t34385: f64, t118354: f64, t120786: f64, t120788: f64, t120792: f64, t120800: f64, t120803: f64, t120807: f64, t123272: f64, t123274: f64, t123282: f64, t123285: f64, t123287: f64, t123290: f64, t1458: f64, t31284: f64, t33195: f64, t671: f64, t8508: f64) -> (f64, f64, f64, f64) {
    let t125982 = t2169 * t8119;
    let t125988 = t1851 * t8927;
    let t125991 = t576 * t34401;
    let t126000 = t34385 * t112;
    let t126004 = t31284 + 0.135e2_f64 * t118354 * t1458 + t8508 + t120786 + 27.0_f64 * t123272 + 27.0_f64 * t123274 + t120788 + t33195 + t120792 + 27.0_f64 * t123282 + t120800 + t120803 + 54.0_f64 * t123285 + 54.0_f64 * t123287 + t120807 + 0.135e2_f64 * t126000 * t671 + 54.0_f64 * t123290;
    (t125982, t125988, t125991, t126004)
}
