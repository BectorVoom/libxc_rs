//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1335/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1335(t14535: f64, t3120: f64, t11606: f64, t14015: f64, t3123: f64, t54084: f64, t11901: f64, t14011: f64, t14046: f64, t15268: f64, t11620: f64, t3139: f64, t37441: f64, t4028: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57060 = t3120 * t14535;
    let t57062 = t14015 * t11606;
    let t57064 = t3123 * t54084;
    let t57066 = t14011 * t11901;
    let t57068 = t14046 * t15268;
    let t57070 = t14011 * t11620;
    let t57073 = t4028 * t3139 * t37441;
    (t57060, t57062, t57064, t57066, t57068, t57070, t57073)
}
