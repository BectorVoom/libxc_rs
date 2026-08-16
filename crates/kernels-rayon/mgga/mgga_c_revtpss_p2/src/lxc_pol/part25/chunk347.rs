//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 347/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk347(t1211: f64, t1214: f64, t139: f64, t221: f64, t462: f64, t461: f64, t1010: f64, t56: f64) -> (f64, f64, f64) {
    let t1215 = t1211 * t1214;
    let t1219 = t221 * t139 * t462;
    let t1221 = t461 * t1219 / 288.0_f64;
    let t1222 = t56 * t1010;
    (t1215, t1221, t1222)
}
