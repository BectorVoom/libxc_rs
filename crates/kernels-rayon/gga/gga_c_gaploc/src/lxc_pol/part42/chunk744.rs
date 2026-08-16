//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 744/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk744(t2547: f64, t279: f64, t481: f64, t122: f64, t2310: f64, t4260: f64, t883: f64, t2321: f64, t28438: f64, t4389: f64, t899: f64, t1415: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29439 = t481 * t2547 * t279;
    let t29874 = t481 * t2310 * t122;
    let t30204 = t4260 * t883;
    let t30733 = t28438 * t2321;
    let t30829 = t4389 * t899;
    let t30830 = t1415 * t30829;
    (t29439, t29874, t30204, t30733, t30829, t30830)
}
