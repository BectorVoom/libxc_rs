//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1387/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1387(t120665: f64, t120672: f64, t120675: f64, t120677: f64, t123205: f64, t123206: f64, t123211: f64, t123213: f64, t123215: f64, t123217: f64, t123220: f64, t1266: f64, t33686: f64, t33756: f64, t652: f64, t671: f64) -> f64 {
    let t123222 = -2.0_f64 * t33756 * t652 * t671 - t1266 * t33686 + 2.0_f64 * t120665 - t120672 + t120675 - t120677 - t123205 - 2.0_f64 * t123206 - 2.0_f64 * t123211 - 2.0_f64 * t123213 - 2.0_f64 * t123215 - 2.0_f64 * t123217 + 3.0_f64 * t123220;
    t123222
}
