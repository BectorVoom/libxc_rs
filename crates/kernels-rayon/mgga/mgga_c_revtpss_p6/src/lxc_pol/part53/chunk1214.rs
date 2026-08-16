//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1214/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1214(t116: f64, t34418: f64, t13426: f64, t8749: f64, t125211: f64, t125213: f64, t125215: f64, t125217: f64, t125223: f64, t1502: f64, t28050: f64, t32791: f64, t33578: f64, t33580: f64, t33583: f64, t4246: f64, t671: f64, t7586: f64, t8756: f64) -> (f64, f64) {
    let t129270 = t34418 * t116;
    let t129273 = t13426 * t8749;
    let t129275 = -2.0_f64 * t129270 * t671 - t1502 * t32791 - 2.0_f64 * t28050 * t7586 - t4246 * t8756 - t125211 - t125213 - t125215 - t125217 + t125223 - 2.0_f64 * t129273 - t33578 - t33580 - t33583;
    (t129270, t129275)
}
