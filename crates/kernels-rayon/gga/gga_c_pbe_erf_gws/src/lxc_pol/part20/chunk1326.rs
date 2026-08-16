//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1326/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1326(t3123: f64, t9127: f64, t11548: f64, t14007: f64, t12015: f64, t14031: f64, t11501: f64, t14567: f64, t6608: f64, t55486: f64, t56954: f64, t56956: f64, t56958: f64, t56960: f64, t56962: f64, t56964: f64, t56966: f64) -> f64 {
    let t56968 = t3123 * t9127;
    let t56970 = t14007 * t11548;
    let t56972 = t14031 * t12015;
    let t56975 = t6608 * t11501 * t14567;
    let t56977 = t56954 / 24.0_f64 - t56956 / 48.0_f64 + t56958 / 128.0_f64 - t56960 / 48.0_f64 + t56962 / 96.0_f64 - t56964 / 384.0_f64 + 5.0_f64 / 96.0_f64 * t56966 + t56968 / 24.0_f64 + t56970 / 384.0_f64 - t56972 / 384.0_f64 + t56975 / 96.0_f64 + t55486;
    t56977
}
