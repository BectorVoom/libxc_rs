//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 773/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk773(t1452: f64, t153: f64, t542: f64, t1457: f64, t242: f64, t1365: f64, t745: f64, t1464: f64, t366: f64, t5: f64, t168: f64, t270: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5580 = t153 * t542 * t1452;
    let t5582 = t1457 * t242;
    let t5585 = t153 * t1365 * t745;
    let t5588 = 0.50257692321302641125e0_f64 * t1464 * t242;
    let t5589 = t5 * t366;
    let t5592 = 0.19455129084526283664e0_f64 * t168 * t5589 * t270;
    (t5580, t5582, t5585, t5588, t5589, t5592)
}
