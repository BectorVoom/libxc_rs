//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2695/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2695(t1353: f64, t4144: f64, t14304: f64, t4147: f64, t13674: f64, t13872: f64, t1448: f64, t39528: f64, t39531: f64, t4139: f64, t4140: f64, t48228: f64, t48231: f64, t48232: f64, t48234: f64, t48236: f64, t48238: f64, t5536: f64, t5541: f64) -> f64 {
    let t49560 = t4144 * t1353;
    let t49564 = t14304 * t4147;
    let t49571 = 18.0_f64 * t13674 * t4139 * t49560 + 18.0_f64 * t13872 * t4140 * t5536 - 3.0_f64 * t1448 * t49564 * t5541 - t39528 + t39531 + t48228 + t48231 - t48232 - t48234 + t48236 + t48238;
    t49571
}
