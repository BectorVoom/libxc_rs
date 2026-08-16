//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2286/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2286(t24428: f64, t24470: f64, t300: f64, t20895: f64, t5184: f64, t1196: f64, t24214: f64, t24217: f64, t24219: f64, t24223: f64, t24255: f64, t24257: f64, t24259: f64, t24261: f64, t24264: f64, t24326: f64, t24329: f64) -> (f64, f64, f64, f64) {
    let t24472 = t300 * (t24428 + t24470);
    let t24473 = t20895 * t5184;
    let t24475 = 0.51947577317044391277e2_f64 * t1196 * t24473;
    let t24476 = -t24214 + t24217 - t24219 + t24223 + t24255 + t24257 + t24259 + t24261 - t24264 + t24326 + t24329 + t24472 - t24475;
    (t24472, t24473, t24475, t24476)
}
