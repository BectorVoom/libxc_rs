//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1220/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1220(t15651: f64, t191: f64, t22: f64, t364: f64, t369: f64, t371: f64, t21419: f64, t2168: f64, t3139: f64, t875: f64, t6222: f64, t6484: f64) -> (f64, f64, f64) {
    let t21647 = 13685.0_f64 / 31104.0_f64 * t364 / t22 / t15651 * t191 * t369 * t371;
    let t21651 = t2168 * t3139 * t21419 * t875 / 24.0_f64;
    let t21652 = t6484 * t6222;
    (t21647, t21651, t21652)
}
