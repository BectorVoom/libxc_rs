//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 704/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk704(t2034: f64, t7315: f64, t2014: f64, t118: f64, t1310: f64, t1453: f64, t1932: f64, t2007: f64, t2011: f64, t508: f64, t569: f64, t649: f64, t651: f64, t671: f64, t6983: f64, t6985: f64, t6990: f64, t6992: f64, t6995: f64, t7005: f64, t7007: f64, t7221: f64, t7231: f64, t7236: f64, t7241: f64, t7314: f64) -> (f64, f64) {
    let t7316 = t2034 * t7315;
    let t7317 = t2014 * t7316;
    let t7318 = -t118 * t7221 - t1310 * t1932 + t1453 * t2011 - t2007 * t649 - t508 * t6983 + t569 * t7231 - 2.0_f64 * t651 * t7007 - 2.0_f64 * t671 * t6985 - t6990 - t6992 - t6995 - t7005 + t7236 + t7241 + t7314 - t7317;
    (t7316, t7318)
}
