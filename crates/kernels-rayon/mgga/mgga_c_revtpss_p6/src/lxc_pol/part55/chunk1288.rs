//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1288/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1288(t128517: f64, t128519: f64, t128521: f64, t128528: f64, t128531: f64, t128533: f64, t128535: f64, t128537: f64, t130929: f64, t2163: f64, t2322: f64, t28683: f64, t28935: f64, t34824: f64, t4254: f64, t651: f64, t671: f64, t7683: f64, t7983: f64, t8764: f64) -> f64 {
    let t131000 = -2.0_f64 * t2163 * t28683 * t651 - 2.0_f64 * t651 * t7683 * t7983 - 2.0_f64 * t130929 * t671 - 2.0_f64 * t2322 * t34824 + 3.0_f64 * t28935 * t8764 - 2.0_f64 * t34824 * t4254 - t128517 - t128519 - t128521 + t128528 + t128531 - t128533 - t128535 - t128537;
    t131000
}
