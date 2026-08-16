//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1377/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1377(t15485: f64, t840: f64, t56855: f64, t56857: f64, t56859: f64, t56861: f64, t56863: f64, t56865: f64, t56867: f64, t56869: f64, t56871: f64, t56873: f64, t56877: f64, t56880: f64) -> (f64, f64) {
    let t58581 = t840 * t15485;
    let t58596 = 7.0_f64 / 144.0_f64 * t56855 + t56857 / 12.0_f64 - t56859 / 96.0_f64 - t56861 / 96.0_f64 - t56863 / 384.0_f64 - t56865 / 384.0_f64 + t56867 / 96.0_f64 - t56869 / 48.0_f64 + t56871 / 96.0_f64 + t56873 / 96.0_f64 + t56877 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t56880;
    (t58581, t58596)
}
