//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1318/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1318(t14538: f64, t3792: f64, t51328: f64, t56855: f64, t56857: f64, t56859: f64, t56861: f64, t56863: f64, t56865: f64, t56867: f64, t56869: f64, t56871: f64, t56873: f64, t56877: f64) -> f64 {
    let t56880 = t14538 * t51328 * t3792;
    let t56882 = 7.0_f64 / 288.0_f64 * t56855 + t56857 / 24.0_f64 - t56859 / 192.0_f64 - t56861 / 192.0_f64 - t56863 / 768.0_f64 - t56865 / 768.0_f64 + t56867 / 192.0_f64 - t56869 / 96.0_f64 + t56871 / 192.0_f64 + t56873 / 192.0_f64 + t56877 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t56880;
    t56882
}
