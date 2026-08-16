//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 683/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk683(t5484: f64, t642: f64, t639: f64, t1724: f64, t1791: f64, t661: f64, t1621: f64, t213: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5485 = t642 * t5484;
    let t5487 = 4.0_f64 / 45.0_f64 * t639 * t5485;
    let t5489 = t1791 * t661 * t1724;
    let t5490 = t1621 * t5489;
    let t5492 = 4.0_f64 / 5.0_f64 * t639 * t5490;
    let t5493 = t9 * t213;
    (t5485, t5487, t5489, t5490, t5492, t5493)
}
