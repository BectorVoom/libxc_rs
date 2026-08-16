//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1251/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1251(t28711: f64, t8634: f64, t2089: f64, t28042: f64, t651: f64, t2322: f64, t34028: f64, t128528: f64, t128531: f64, t128533: f64, t128535: f64, t128537: f64, t128539: f64, t13426: f64, t18227: f64, t28053: f64, t32410: f64, t4248: f64, t7359: f64, t8637: f64) -> f64 {
    let t128543 = 2.0_f64 * t8634 * t28711;
    let t128552 = 2.0_f64 * t651 * t2089 * t28042;
    let t128554 = 2.0_f64 * t2322 * t34028;
    let t128555 = -2.0_f64 * t13426 * t8637 - 2.0_f64 * t18227 * t8637 - 2.0_f64 * t28053 * t7359 - 2.0_f64 * t32410 * t4248 + t128528 + t128531 - t128533 - t128535 - t128537 - t128539 - t128543 - t128552 - t128554;
    t128555
}
