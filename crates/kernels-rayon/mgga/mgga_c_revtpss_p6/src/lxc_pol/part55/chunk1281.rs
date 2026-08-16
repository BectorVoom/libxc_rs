//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1281/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1281(t104115: f64, t111734: f64, t128198: f64, t128200: f64, t128204: f64, t128211: f64, t128219: f64, t128223: f64, t128225: f64, t128228: f64, t128231: f64, t128235: f64, t128236: f64, t2056: f64, t5787: f64, t8897: f64) -> f64 {
    let t130907 = -2.0_f64 * t104115 * t2056 - 2.0_f64 * t111734 * t2056 + t5787 * t8897 + t128198 - t128200 - t128204 - t128211 - t128219 + t128223 + t128225 - t128228 + t128231 - t128235 - t128236;
    t130907
}
