//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 967/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk967(t28280: f64, t572: f64, t1461: f64, t1918: f64, t2040: f64, t28246: f64, t28257: f64, t28259: f64, t28261: f64, t28263: f64, t28267: f64, t28270: f64, t28273: f64, t28275: f64, t28279: f64, t573: f64, t5802: f64, t5805: f64, t7324: f64, t7944: f64) -> f64 {
    let t28282 = 3.0_f64 * t572 * t28280;
    let t28283 = 3.0_f64 * t1461 * t7944 + 3.0_f64 * t1918 * t7324 + 6.0_f64 * t2040 * t5802 + 3.0_f64 * t2040 * t5805 + t28246 * t573 + t28257 + t28259 + t28261 + t28263 + t28267 + t28270 + t28273 + t28275 + t28279 + t28282;
    t28283
}
