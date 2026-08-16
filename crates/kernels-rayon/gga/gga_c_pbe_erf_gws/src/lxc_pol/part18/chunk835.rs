//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 835/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk835(t561: f64, t7959: f64, t2575: f64, t4934: f64, t1620: f64, t2826: f64, t583: f64, t1076: f64, t1365: f64, t153: f64, t2513: f64, t414: f64) -> (f64, f64, f64, f64, f64) {
    let t7960 = t561 * t7959;
    let t7966 = t4934 * t2575;
    let t7968 = 32.0_f64 / 135.0_f64 * t1620 * t7966;
    let t7970 = 8.0_f64 / 45.0_f64 * t2826 * t583;
    let t7981 = t153 * t1365 * t1076;
    let t7983 = t414 * t2513;
    (t7960, t7968, t7970, t7981, t7983)
}
