//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1121/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1121(t2409: f64, t6149: f64, t14121: f64, t274: f64, t837: f64, t850: f64, t851: f64, t833: f64, t3955: f64, t894: f64, t3975: f64, t9521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14122 = t2409 * t6149;
    let t14123 = t14121 * t14122;
    let t14125 = t274 * t837;
    let t14127 = t850 * t851 * t14125;
    let t14128 = t14127 * t833;
    let t14129 = 7.0_f64 / 144.0_f64 * t14128;
    let t14130 = t3955 * t894;
    let t14131 = 7.0_f64 / 144.0_f64 * t14130;
    let t14132 = t3975 * t9521;
    (t14122, t14123, t14125, t14127, t14128, t14129, t14130, t14131, t14132)
}
