//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1275/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1275(t14011: f64, t9681: f64, t14015: f64, t9527: f64, t51312: f64, t9035: f64, t14570: f64, t6538: f64, t3123: f64, t51430: f64, t14007: f64, t9438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54144 = t14011 * t9681;
    let t54146 = t14015 * t9527;
    let t54148 = t9035 * t51312;
    let t54150 = t6538 * t14570;
    let t54152 = t3123 * t51430;
    let t54154 = t14007 * t9438;
    (t54144, t54146, t54148, t54150, t54152, t54154)
}
