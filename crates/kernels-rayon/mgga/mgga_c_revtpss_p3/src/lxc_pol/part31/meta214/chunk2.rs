//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 965/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk965(t225: f64, t5638: f64, t5642: f64, t539: f64, t73: f64, t1412: f64, t1868: f64) -> (f64, f64, f64) {
    let t5644 = (t5638 + t5642) * t225;
    let t5650 = t539 * t73;
    let t5651 = t1412 * t1868;
    (t5644, t5650, t5651)
}
