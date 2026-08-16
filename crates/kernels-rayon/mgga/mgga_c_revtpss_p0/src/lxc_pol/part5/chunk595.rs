//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 595/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk595(t2852: f64, t3252: f64, t1071: f64, t342: f64, t1077: f64, t384: f64, t225: f64) -> (f64, f64, f64, f64) {
    let t3253 = t3252 * t2852;
    let t3264 = t342 * t1071;
    let t3268 = 1.0_f64 / t1077 / t384;
    let t3269 = t225 * t3268;
    (t3253, t3264, t3268, t3269)
}
