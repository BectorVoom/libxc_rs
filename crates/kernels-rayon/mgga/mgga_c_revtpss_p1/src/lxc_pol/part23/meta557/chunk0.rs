//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2116/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2116(t1444: f64, t6895: f64, t9657: f64, t22307: f64, t225: f64, t212: f64, t6888: f64, t1358: f64, t689: f64, t1357: f64, t6896: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22432 = t6895 * t1444;
    let t22433 = t9657 * t22432;
    let t22441 = t22307 * t225;
    let t22445 = t212 * t6888;
    let t22446 = t22445 * t1358;
    let t22447 = t689 * t22446;
    let t22449 = t1357 * t6896;
    let t22450 = t689 * t22449;
    let t22452 = t6895 * t72;
    (t22432, t22433, t22441, t22445, t22446, t22447, t22449, t22450, t22452)
}
