//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1296/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1296(t120114: f64, t120171: f64, t120176: f64, t120658: f64, t120663: f64, t120672: f64, t120677: f64, t120683: f64, t123195: f64, t123199: f64, t123205: f64, t123206: f64, t123211: f64, t123213: f64, t123215: f64, t123217: f64, t123220: f64, t125903: f64, t510: f64) -> f64 {
    let t125951 = -t125903 * t510 - t120114 + t120171 - t120176 + t120658 + t120663 - t120672 - t120677 - t120683 + 4.0_f64 * t123195 + 12.0_f64 * t123199 - 2.0_f64 * t123205 - 4.0_f64 * t123206 - 4.0_f64 * t123211 - 4.0_f64 * t123213 - 4.0_f64 * t123215 - 4.0_f64 * t123217 + 6.0_f64 * t123220;
    t125951
}
