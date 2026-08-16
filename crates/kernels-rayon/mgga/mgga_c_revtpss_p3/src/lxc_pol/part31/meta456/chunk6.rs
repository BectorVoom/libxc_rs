//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1655/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1655(t20721: f64, t247: f64, t3719: f64, t3670: f64, t5390: f64, t1225: f64, t18281: f64, t1012: f64, t1010: f64, t5843: f64, t5378: f64, t5381: f64) -> (f64, f64, f64, f64, f64) {
    let t21200 = t247 * t3719 * t20721;
    let t21203 = t3670 * t5390;
    let t21209 = t1225 * t18281;
    let t21210 = t1012 * t21209;
    let t21213 = t5843 * t1010;
    let t21216 = t5381 * t5378;
    (t21200, t21203, t21210, t21213, t21216)
}
