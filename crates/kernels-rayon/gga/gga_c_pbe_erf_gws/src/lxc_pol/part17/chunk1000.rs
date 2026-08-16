//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1000/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1000(t3142: f64, t8967: f64, t3172: f64, t6484: f64, t2206: f64, t3195: f64, t8574: f64, t858: f64, t886: f64, t884: f64, t1114: f64, t6677: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8969 = 7.0_f64 / 72.0_f64 * t8967 * t3142;
    let t8971 = 7.0_f64 / 144.0_f64 * t6484 * t3172;
    let t8973 = 7.0_f64 / 72.0_f64 * t2206 * t3195;
    let t8975 = t886 * t858 * t8574;
    let t8977 = t884 * t8975 / 48.0_f64;
    let t8978 = t1114 * t6677;
    (t8969, t8971, t8973, t8975, t8977, t8978)
}
