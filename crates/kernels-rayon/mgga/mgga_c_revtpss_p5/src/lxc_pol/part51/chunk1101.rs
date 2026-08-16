//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1101/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1101(t32322: f64, t7937: f64, t13648: f64, t2014: f64, t8595: f64, t125483: f64, t125486: f64, t125488: f64, t125491: f64, t125495: f64, t125497: f64, t125499: f64, t125500: f64, t125502: f64, t125505: f64, t125507: f64, t125510: f64, t125512: f64, t125514: f64, t125515: f64, t125517: f64, t125521: f64) -> f64 {
    let t125522 = t32322 * t7937;
    let t125525 = t2014 * t8595 * t13648;
    let t125526 = -t125483 + t125486 - t125488 - t125491 + t125495 + 12.0_f64 * t125497 - t125499 - 2.0_f64 * t125500 + 6.0_f64 * t125502 - t125505 - t125507 + t125510 + t125512 - t125514 - 4.0_f64 * t125515 - 4.0_f64 * t125517 - t125521 - 2.0_f64 * t125522 - t125525;
    t125526
}
