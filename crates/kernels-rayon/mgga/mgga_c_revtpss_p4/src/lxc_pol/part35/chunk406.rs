//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 406/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk406(t112: f64, t2289: f64, t111: f64, t654: f64, t99: f64, t107: f64, t200: f64, t202: f64, t205: f64, t262: f64, t198: f64, t206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2335 = 11.0_f64 / 9.0_f64 * t2289 * t112;
    let t2339 = 1.0_f64 / t654 / t111;
    let t2349 = 1.0_f64 / t99;
    let t2357 = 1.0_f64 / t107;
    let t2375 = 1.0_f64 / t200;
    let t2382 = 1.0_f64 / t202;
    let t2393 = t205 * t262;
    let t2403 = t198 * t206;
    (t2335, t2339, t2349, t2357, t2375, t2382, t2393, t2403)
}
