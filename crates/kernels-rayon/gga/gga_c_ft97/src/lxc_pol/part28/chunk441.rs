//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 441/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk441(t574: f64, t605: f64, t6699: f64, t1384: f64, t3578: f64, t144: f64, t1053: f64) -> (f64, f64, f64) {
    let t6701 = t574 * t605 * t6699;
    let t6704 = t3578 * t1384;
    let t6705 = t144 * t6704;
    let t6708 = t1384 * t1053;
    (t6701, t6705, t6708)
}
