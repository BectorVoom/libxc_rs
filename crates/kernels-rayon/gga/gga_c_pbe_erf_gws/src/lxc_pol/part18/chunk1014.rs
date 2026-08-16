//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1014/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1014(t10013: f64, t11310: f64, t11312: f64, t11314: f64, t3772: f64, t817: f64, t3776: f64, t745: f64, t1076: f64, t2848: f64, t3373: f64, t10272: f64, t2102: f64, t2107: f64, t3030: f64, t3033: f64, t323: f64, t6089: f64, t6096: f64, t818: f64, t9147: f64, t9150: f64) -> (f64, f64) {
    let t11316 = t10013 + t11310 + t11312 + t11314;
    let t11318 = t3772 * t817;
    let t11328 = t3776 * t745;
    let t11331 = t1076 * t2848;
    let t11335 = t3373 * t745;
    let t11339 = -t10272 * t818 - 2.0_f64 * t1076 * t9147 + t11316 * t323 - t11318 * t745 - 6.0_f64 * t11328 * t6096 + 4.0_f64 * t11331 * t2107 + 2.0_f64 * t11335 * t2107 - t2102 * t3373 - 2.0_f64 * t2848 * t3030 + 4.0_f64 * t3033 * t9150 + 2.0_f64 * t3776 * t6089;
    (t11316, t11339)
}
