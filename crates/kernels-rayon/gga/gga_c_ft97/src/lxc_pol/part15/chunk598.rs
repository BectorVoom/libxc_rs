//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 598/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk598(t10397: f64, t2404: f64, t798: f64, t295: f64, t9577: f64, t2344: f64) -> (f64, f64, f64, f64) {
    let t10398 = 14.0_f64 / 81.0_f64 * t10397;
    let t10409 = t2404 * t798;
    let t10414 = t295 * t9577;
    let t10478 = t2344 * t798;
    (t10398, t10409, t10414, t10478)
}
