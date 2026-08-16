//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1320/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1320(t11521: f64, t14498: f64, t11930: f64, t14015: f64, t11750: f64, t51351: f64, t11444: f64, t11938: f64, t51201: f64, t54015: f64, t56883: f64, t56885: f64, t56887: f64, t56889: f64, t56892: f64, t56894: f64) -> f64 {
    let t56896 = t14498 * t11521;
    let t56898 = t14015 * t11930;
    let t56901 = t51351 * t11750;
    let t56903 = t51351 * t11444;
    let t56905 = t14498 * t11938;
    let t56907 = -t56883 / 96.0_f64 - t56885 / 96.0_f64 - t56887 / 192.0_f64 + t56889 / 48.0_f64 - t56892 / 48.0_f64 + t56894 / 384.0_f64 + t56896 / 256.0_f64 + t54015 - t56898 / 192.0_f64 + 119.0_f64 / 3456.0_f64 * t51201 - t56901 / 192.0_f64 - t56903 / 96.0_f64 - t56905 / 64.0_f64;
    t56907
}
