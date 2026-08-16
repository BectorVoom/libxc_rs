//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2409/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2409(t11200: f64, t3286: f64, t3046: f64, t4995: f64, t3057: f64, t3143: f64, t42859: f64, t342: f64, t16551: f64, t994: f64, t16558: f64, t16505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43446 = t11200 * t3286;
    let t43453 = t3046 * t4995;
    let t43456 = t3057 * t4995;
    let t43471 = t42859 * t3143;
    let t43472 = t342 * t43471;
    let t43520 = t994 * t16551;
    let t43524 = t994 * t16558;
    let t43528 = t994 * t16505;
    (t43446, t43453, t43456, t43471, t43472, t43520, t43524, t43528)
}
