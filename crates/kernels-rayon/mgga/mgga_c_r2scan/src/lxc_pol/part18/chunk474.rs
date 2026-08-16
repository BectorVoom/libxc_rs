//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 474/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk474(t2452: f64, t41: f64, t406: f64, t899: f64, t2267: f64, t910: f64, t2266: f64, t879: f64, t955: f64, t1776: f64, t1782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2453 = t41 * t2452;
    let t2454 = t406 * t899;
    let t2455 = 4.0_f64 * t2454;
    let t2456 = t2267 * t910;
    let t2457 = t2266 * t2456;
    let t2458 = 3.0_f64 * t2457;
    let t2460 = t879 * t955;
    let t2461 = t1776 - t1782;
    (t2453, t2454, t2455, t2458, t2460, t2461)
}
