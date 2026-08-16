//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3823/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3823(t73493: f64, t13625: f64, t13674: f64, t1868: f64, t1907: f64, t198: f64, t33596: f64, t39799: f64, t39807: f64, t39813: f64, t4139: f64, t47059: f64, t49647: f64, t530: f64, t73418: f64, t73474: f64, t73477: f64, t73482: f64, t73488: f64) -> (f64, f64) {
    let t73494 = 0.36622894612013090108e-3_f64 * t73493;
    let t73495 = -24.0_f64 * t13625 * t1907 * t198 * t33596 * t530 + 12.0_f64 * t13674 * t4139 * t73488 + 6.0_f64 * t1868 * t4139 * t49647 + t39799 + t39807 - t39813 + t47059 + t73418 + t73474 + t73477 - t73482 - t73494;
    (t73494, t73495)
}
