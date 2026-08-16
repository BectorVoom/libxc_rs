//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1378/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1378(t54026: f64, t55432: f64, t56883: f64, t56885: f64, t56887: f64, t56889: f64, t56892: f64, t56894: f64, t56896: f64, t56898: f64, t56901: f64, t56903: f64, t56905: f64) -> f64 {
    let t58608 = -t56883 / 48.0_f64 - t56885 / 48.0_f64 - t56887 / 96.0_f64 + t56889 / 24.0_f64 - t56892 / 24.0_f64 + t56894 / 192.0_f64 + t56896 / 128.0_f64 + t55432 - t56898 / 96.0_f64 + t54026 - t56901 / 96.0_f64 - t56903 / 48.0_f64 - t56905 / 32.0_f64;
    t58608
}
