//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1381/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1381(t54126: f64, t56954: f64, t56956: f64, t56958: f64, t56960: f64, t56962: f64, t56964: f64, t56966: f64, t56968: f64, t56970: f64, t56972: f64, t56975: f64) -> f64 {
    let t58645 = t56954 / 12.0_f64 - t56956 / 24.0_f64 + t56958 / 64.0_f64 - t56960 / 24.0_f64 + t56962 / 48.0_f64 - t56964 / 192.0_f64 + 5.0_f64 / 48.0_f64 * t56966 + t56968 / 12.0_f64 + t56970 / 192.0_f64 - t56972 / 192.0_f64 + t56975 / 48.0_f64 + 119.0_f64 / 864.0_f64 * t54126;
    t58645
}
