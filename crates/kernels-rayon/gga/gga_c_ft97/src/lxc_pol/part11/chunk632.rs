//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 632/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk632(t8659: f64, t8721: f64, t184: f64, t21: f64, t2304: f64, t648: f64, t2299: f64, t3664: f64, t1580: f64, t649: f64, t2300: f64, t363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8722 = t8659 + t8721;
    let t8723 = t8722 * t184;
    let t8724 = t8723 * t21;
    let t8730 = t2304 * t648;
    let t8731 = t8730 * t184;
    let t8732 = t8731 * t21;
    let t8738 = t2299 * t648;
    let t8739 = t8738 * t3664;
    let t8744 = t649 * t1580;
    let t8751 = t2300 * t363;
    (t8722, t8723, t8724, t8731, t8732, t8738, t8739, t8744, t8751)
}
