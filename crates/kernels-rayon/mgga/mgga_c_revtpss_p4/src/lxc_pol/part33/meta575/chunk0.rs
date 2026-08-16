//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1984/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1984(t1398: f64, t6861: f64, t221: f64, t22274: f64, t22279: f64, t22287: f64, t6843: f64, t1883: f64, t5658: f64, t543: f64, t73820: f64, t6862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73842 = t6861 * t1398;
    let t74419 = t221 * t22274;
    let t74423 = t221 * t22279;
    let t74477 = t221 * t22287;
    let t74700 = t6843 * t1398;
    let t75012 = t1883 * t5658;
    let t75016 = t73842 * t543;
    let t75047 = t73820 * t1398;
    let t75051 = t6862 * t1398;
    (t74419, t74423, t74477, t74700, t75012, t75016, t75047, t75051)
}
