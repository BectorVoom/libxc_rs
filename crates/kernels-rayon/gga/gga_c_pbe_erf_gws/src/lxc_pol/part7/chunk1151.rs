//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1151/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1151(t20599: f64, t6678: f64, t2158: f64, t810: f64, t2407: f64, t858: f64, t8988: f64, t6274: f64, t6684: f64, t824: f64, t19993: f64, t2210: f64, t884: f64) -> (f64, f64, f64, f64, f64) {
    let t20601 = t6678 * t20599 / 16.0_f64;
    let t20602 = t2158 * t810;
    let t20604 = t2407 * t858 * t20602;
    let t20606 = t8988 * t20604 / 2.0_f64;
    let t20607 = t6684 * t6274;
    let t20608 = t824 * t20602;
    let t20615 = t884 * t2210 * t858 * t19993 / 4.0_f64;
    (t20601, t20606, t20607, t20608, t20615)
}
