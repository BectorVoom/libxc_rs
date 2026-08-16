//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1061/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1061(t2345: f64, t3814: f64, t9375: f64, t11732: f64, t858: f64, t867: f64, t866: f64, t11737: f64, t2210: f64, t884: f64, t2164: f64, t3880: f64) -> (f64, f64, f64, f64) {
    let t11901 = t2345 * t9375 * t3814;
    let t11905 = t867 * t858 * t11732;
    let t11907 = t866 * t11905 / 96.0_f64;
    let t11909 = t2210 * t858 * t11737;
    let t11911 = t884 * t11909 / 16.0_f64;
    let t11912 = t2164 * t3880;
    (t11901, t11907, t11911, t11912)
}
