//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1119/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1119(t14054: f64, t14105: f64, t898: f64, t338: f64, t353: f64, t1176: f64, t923: f64, t931: f64, t3985: f64, t376: f64, t911: f64, t2158: f64, t3990: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14106 = t14054 + t14105;
    let t14107 = t898 * t14106;
    let t14109 = t338 * t353 * t14107;
    let t14113 = t1176 * t923 * t931;
    let t14114 = t14113 * t3985;
    let t14115 = 7.0_f64 / 576.0_f64 * t14114;
    let t14116 = t911 * t376;
    let t14118 = t3990 * t14116 * t2158;
    (t14106, t14107, t14109, t14113, t14114, t14115, t14116, t14118)
}
