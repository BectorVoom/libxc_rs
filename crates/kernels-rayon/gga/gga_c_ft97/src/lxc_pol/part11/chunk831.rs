//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 831/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk831(t432: f64, t7973: f64, t1564: f64, t446: f64, t1570: f64, t363: f64, t7745: f64) -> (f64, f64, f64) {
    let t37259 = t7973 * t432;
    let t37261 = t446 * t1564 * t37259;
    let t37264 = t1570 * t7745 * t363;
    (t37259, t37261, t37264)
}
