//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1215/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1215(t2313: f64, t6638: f64, t12076: f64, t19714: f64, t2118: f64, t3074: f64, t6179: f64, t6183: f64, t2134: f64, t20886: f64, t343: f64, t6345: f64, t814: f64) -> (f64, f64, f64, f64, f64) {
    let t21570 = t2313 * t6638;
    let t21577 = 7.0_f64 / 48.0_f64 * t3074 * t2118 * t19714 * t12076;
    let t21578 = t6183 * t6179;
    let t21579 = t2134 * t21578;
    let t21580 = 7.0_f64 / 24.0_f64 * t21579;
    let t21581 = t20886 * t343;
    let t21586 = t6345 * t814;
    (t21570, t21577, t21580, t21581, t21586)
}
