//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 632/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk632(t1734: f64, t420: f64, t301: f64, t402: f64, t5506: f64, t1396: f64, t1402: f64, t1404: f64, t1407: f64, t153: f64, t155: f64, t1828: f64, t1832: f64, t1835: f64, t400: f64, t403: f64, t519: f64, t521: f64, t6039: f64, t6045: f64, t6053: f64, t6056: f64) -> f64 {
    let t6061 = t420 * t1734;
    let t6062 = t6061 * t301;
    let t6065 = t402 * t5506;
    let t6068 = 6.0_f64 * t1396 * t521 + 60.0_f64 * t1402 * t6053 - 24.0_f64 * t1402 * t6056 - 12.0_f64 * t1402 * t6062 - 24.0_f64 * t1404 * t6045 + 6.0_f64 * t1407 * t519 + 3.0_f64 * t153 * t6065 - t155 * t6039 + 3.0_f64 * t1828 * t403 - 12.0_f64 * t1832 * t400 + 3.0_f64 * t1835 * t400;
    t6068
}
