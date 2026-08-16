//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1069/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1069(t9176: f64, t1133: f64, t745: f64, t343: f64, t1123: f64, t2255: f64, t1076: f64, t874: f64, t274: f64, t3165: f64, t11964: f64, t254: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12005 = 35.0_f64 / 108.0_f64 * t9176;
    let t12006 = t745 * t1133;
    let t12007 = t12006 * t343;
    let t12008 = t1123 * t12007;
    let t12009 = t2255 * t12008;
    let t12013 = t1076 * t874 * t343;
    let t12014 = t1123 * t12013;
    let t12015 = t2255 * t12014;
    let t12019 = t274 * t3165 * t343;
    let t12020 = t1123 * t12019;
    let t12021 = t2255 * t12020;
    let t12024 = t254 * t11964;
    (t12005, t12008, t12009, t12014, t12015, t12020, t12021, t12024)
}
