//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 971/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk971(t10968: f64, t184: f64, t564: f64, t3450: f64, t582: f64, t561: f64, t5513: f64, t1006: f64, t2786: f64, t3425: f64, t610: f64, t1827: f64) -> (f64, f64, f64, f64, f64) {
    let t10969 = t10968 * t184;
    let t10971 = 4.0_f64 / 15.0_f64 * t10969 * t564;
    let t10972 = t582 * t3450;
    let t10973 = t561 * t10972;
    let t10974 = 8.0_f64 / 45.0_f64 * t10973;
    let t10975 = 4.0_f64 / 135.0_f64 * t5513;
    let t10977 = 4.0_f64 / 15.0_f64 * t1006 * t2786;
    let t10978 = t3425 * t610;
    let t10979 = t1827 * t10978;
    (t10971, t10974, t10975, t10977, t10979)
}
