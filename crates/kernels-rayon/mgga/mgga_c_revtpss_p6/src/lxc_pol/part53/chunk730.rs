//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 730/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk730(t670: f64, t7226: f64, t7228: f64, t7230: f64, t7584: f64, t7586: f64, t118: f64, t1310: f64, t1453: f64, t2127: f64, t2163: f64, t2165: f64, t508: f64, t569: f64, t649: f64, t651: f64, t671: f64, t6990: f64, t6992: f64, t6995: f64, t7005: f64, t7236: f64, t7241: f64, t7314: f64, t7317: f64, t7591: f64, t7683: f64) -> (f64, f64) {
    let t7687 = 2.0_f64 * t670 * t7586 + t7226 + t7228 + t7230 + t7584;
    let t7690 = -t118 * t7683 - t1310 * t2127 + t1453 * t2165 - t2163 * t649 - t508 * t7584 + t569 * t7687 - 2.0_f64 * t651 * t7591 - 2.0_f64 * t671 * t7586 - t6990 - t6992 - t6995 - t7005 + t7236 + t7241 + t7314 - t7317;
    (t7687, t7690)
}
