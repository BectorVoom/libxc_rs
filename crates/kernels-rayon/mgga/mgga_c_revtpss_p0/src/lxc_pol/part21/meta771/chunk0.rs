//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2738/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2738(t1565: f64, t40781: f64, t40488: f64, t4354: f64, t14862: f64, t9775: f64, t268: f64, t40452: f64, t4371: f64, t2662: f64, t40689: f64, t4353: f64) -> (f64, f64, f64, f64, f64) {
    let t50370 = t40781 * t1565;
    let t50372 = t40488 * t4354;
    let t50374 = t9775 * t14862;
    let t50375 = 0.22866142996303859718e-3_f64 * t50374;
    let t50377 = t40452 * t4371 * t268;
    let t50381 = t40689 * t2662 * t4353 * t268;
    (t50370, t50372, t50375, t50377, t50381)
}
