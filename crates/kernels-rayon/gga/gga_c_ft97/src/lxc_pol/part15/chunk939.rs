//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 939/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk939(t1775: f64, t20363: f64, t1882: f64, t20153: f64, t20138: f64, t20124: f64, t37401: f64, t89: f64, t1586: f64, t20098: f64, t20461: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74287 = t1775 * t20363;
    let t74307 = t1882 * t20153;
    let t74374 = t1882 * t20138;
    let t74377 = t89 * t37401 * t20124;
    let t74389 = t1586 * t20098;
    let t74690 = t20461 * t487;
    (t74287, t74307, t74374, t74377, t74389, t74690)
}
