//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1285/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1285(t128274: f64, t128277: f64, t128279: f64, t128280: f64, t128282: f64, t128284: f64, t128287: f64, t128289: f64, t128293: f64, t128294: f64, t128295: f64, t128301: f64, t28653: f64, t33287: f64, t4293: f64, t7591: f64) -> f64 {
    let t130961 = -2.0_f64 * t28653 * t7591 - 2.0_f64 * t33287 * t4293 - t128274 + t128277 + t128279 - t128280 + t128282 - t128284 - t128287 - 2.0_f64 * t128289 - 2.0_f64 * t128293 - 2.0_f64 * t128294 - 2.0_f64 * t128295 - 2.0_f64 * t128301;
    t130961
}
