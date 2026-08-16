//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 783/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk783(t12716: f64, t1827: f64, t587: f64, t3411: f64, t7130: f64, t10424: f64, t950: f64, t1821: f64, t1820: f64, t7580: f64, t1033: f64, t3555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12717 = t1827 * t12716;
    let t12719 = 4.0_f64 / 15.0_f64 * t587 * t12717;
    let t12721 = 16.0_f64 / 15.0_f64 * t7130 * t3411;
    let t12722 = t10424 * t950;
    let t12723 = t1821 * t12722;
    let t12725 = 8.0_f64 / 15.0_f64 * t1820 * t12723;
    let t12726 = 8.0_f64 / 135.0_f64 * t7580;
    let t12728 = 2.0_f64 / 5.0_f64 * t1033 * t3555;
    (t12717, t12719, t12721, t12722, t12723, t12725, t12726, t12728)
}
