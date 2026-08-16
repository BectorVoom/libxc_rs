//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 768/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk768(t10402: f64, t666: f64, t89: f64, t835: f64, t9592: f64, t446: f64, t2404: f64, t798: f64, t2405: f64, t824: f64, t295: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10404 = t89 * t666 * t10402;
    let t10406 = t835 * t9592;
    let t10407 = t446 * t10406;
    let t10409 = t2404 * t798;
    let t10410 = t2405 * t824;
    let t10411 = t10409 * t10410;
    let t10412 = t446 * t10411;
    let t10414 = t295 * t9577;
    (t10404, t10406, t10407, t10409, t10410, t10411, t10412, t10414)
}
