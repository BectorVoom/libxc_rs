//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 842/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk842(t542: f64, t974: f64, t496: f64, t2900: f64, t513: f64, t1576: f64, t981: f64, t1563: f64, t9: f64, t155: f64, t506: f64, t2911: f64, t2913: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8199 = t542 * t974;
    let t8200 = t496 * t8199;
    let t8206 = t2900 * t513;
    let t8209 = t981 * t1576;
    let t8231 = t9 * t1563;
    let t8236 = t155 * t506;
    let t8238 = t2911 * t8236 * t2913;
    (t8199, t8200, t8206, t8209, t8231, t8238)
}
