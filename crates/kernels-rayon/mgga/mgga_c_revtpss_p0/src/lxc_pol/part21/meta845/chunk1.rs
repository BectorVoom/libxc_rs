//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3163/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3163(t12227: f64, t12228: f64, t5108: f64, t3451: f64, t5117: f64, t3383: f64, t5060: f64, t3386: f64, t12247: f64, t1719: f64, t12249: f64, t1756: f64, t3521: f64) -> (f64, f64, f64, f64, f64) {
    let t58333 = 0.57895126195293126241e3_f64 * t12227 * t5108 * t12228;
    let t58336 = t5117 * t3451;
    let t58339 = t5060 * t3383;
    let t58341 = 6.0_f64 * t58339 * t3386;
    let t58342 = t1719 * t12247;
    let t58344 = 0.96491876992155210402e2_f64 * t58342 * t12249;
    let t58345 = t3521 * t1756;
    (t58333, t58336, t58341, t58344, t58345)
}
