//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1020/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1020(t9182: f64, t1123: f64, t6491: f64, t850: f64, t860: f64, t2145: f64, t3039: f64, t2150: f64, t3180: f64, t6322: f64, t3131: f64, t6523: f64, t6524: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9183 = 35.0_f64 / 432.0_f64 * t9182;
    let t9185 = t850 * t1123 * t6491;
    let t9187 = t9185 * t860 / 96.0_f64;
    let t9188 = t3039 * t2145;
    let t9190 = t9188 * t2150 / 24.0_f64;
    let t9192 = t6322 * t3180 / 48.0_f64;
    let t9194 = t6523 * t3131 * t6524;
    (t9183, t9185, t9187, t9190, t9192, t9194)
}
