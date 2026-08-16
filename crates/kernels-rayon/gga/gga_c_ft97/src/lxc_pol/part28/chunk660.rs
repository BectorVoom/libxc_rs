//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 660/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk660(t2179: f64, t26520: f64, t1053: f64, t5968: f64, t1384: f64, t3565: f64, t6708: f64, t9276: f64, t12664: f64, t5956: f64, t1017: f64, t614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26521 = t2179 * t26520;
    let t26523 = t5968 * t1053;
    let t26524 = t2179 * t26523;
    let t26526 = t1384 * t3565;
    let t26527 = t2179 * t26526;
    let t26529 = t9276 * t6708;
    let t26531 = t12664 * t5956;
    let t26533 = t614 * t1017;
    (t26521, t26523, t26524, t26526, t26527, t26529, t26531, t26533)
}
