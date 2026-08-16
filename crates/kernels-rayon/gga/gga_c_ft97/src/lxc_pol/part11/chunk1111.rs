//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1111/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1111(t2404: f64, t2680: f64, t2405: f64, t2682: f64, t446: f64, t2409: f64, t2739: f64, t2665: f64, t824: f64, t9578: f64, t10409: f64, t10411: f64, t1882: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43350 = t2404 * t2680;
    let t43351 = t2405 * t2682;
    let t43353 = t446 * t43350 * t43351;
    let t43355 = t2409 * t2739;
    let t43357 = t446 * t2665 * t43355;
    let t43359 = t9578 * t824;
    let t43361 = t446 * t10409 * t43359;
    let t43363 = t1882 * t10411;
    (t43351, t43353, t43355, t43357, t43359, t43361, t43363)
}
