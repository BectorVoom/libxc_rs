//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 921/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk921(t1256: f64, t1804: f64, t1786: f64, t1230: f64, t1803: f64, t225: f64, t5216: f64, t480: f64, t1796: f64, t3172: f64, t1247: f64, t1263: f64, t3367: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5254 = t1804 * t1256;
    let t5256 = t1786 * t1256;
    let t5258 = t1230 * t1803;
    let t5261 = t5216 * t225;
    let t5262 = t5261 * t480;
    let t5265 = t3172 * t1796;
    let t5266 = t1247 * t5265;
    let t5268 = t1263 * t3367;
    (t5254, t5256, t5258, t5261, t5262, t5265, t5266, t5268)
}
