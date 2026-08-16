//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1059/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1059(t32629: f64, t7238: f64, t2014: f64, t2107: f64, t32113: f64, t7235: f64, t8718: f64, t7536: f64, t8717: f64, t2007: f64, t2052: f64, t2108: f64, t32322: f64, t32619: f64, t32620: f64, t32621: f64, t32627: f64, t32628: f64, t651: f64, t7221: f64, t7357: f64, t7537: f64, t7539: f64, t8568: f64) -> (f64, f64, f64, f64) {
    let t32630 = t32629 * t7238;
    let t32632 = 3.0_f64 * t2014 * t32630;
    let t32633 = t2107 * t32113;
    let t32634 = t2014 * t32633;
    let t32635 = t7235 * t8718;
    let t32636 = t7536 * t8717;
    let t32637 = t2014 * t32636;
    let t32638 = -t2007 * t7357 - t2052 * t7221 + t2108 * t32322 - 2.0_f64 * t32621 * t651 + t7537 * t8568 - t7539 * t8568 - t32619 - t32620 + t32627 + t32628 + t32632 - t32634 - t32635 - t32637;
    (t32630, t32633, t32636, t32638)
}
