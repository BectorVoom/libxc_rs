//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 300/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk300(t19: f64, t885: f64, t810: f64, t858: f64, t884: f64, t862: f64, t879: f64, t882: f64, t339: f64) -> (f64, f64, f64, f64, f64) {
    let t886 = t885 * t19;
    let t887 = t858 * t810;
    let t888 = t886 * t887;
    let t890 = t884 * t888 / 48.0_f64;
    let t891 = t862 - t879 - t882 - t890;
    let t892 = t339 * t891;
    (t886, t888, t890, t891, t892)
}
