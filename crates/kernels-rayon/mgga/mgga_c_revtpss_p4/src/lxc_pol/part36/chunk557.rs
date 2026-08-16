//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 557/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk557(t1261: f64, t5378: f64, t1260: f64, t1785: f64, t3670: f64, t1802: f64, t369: f64, t475: f64, t467: f64, t1811: f64, t460: f64, t1284: f64, t1770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5379 = t1261 * t5378;
    let t5381 = t1785 * t1260;
    let t5384 = t3670 * t1260;
    let t5389 = t1802 * t369;
    let t5390 = t475 * t5389;
    let t5391 = t467 * t5390;
    let t5417 = t460 * t1811;
    let t5436 = t1770 * t1284;
    (t5379, t5381, t5384, t5389, t5390, t5391, t5417, t5436)
}
