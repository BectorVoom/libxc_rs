//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1276/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1276(t14069: f64, t9111: f64, t14064: f64, t3108: f64, t14031: f64, t9348: f64, t14011: f64, t9666: f64, t14538: f64, t51329: f64, t4028: f64, t9131: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54158 = t9111 * t14069;
    let t54160 = t3108 * t14064;
    let t54162 = t14031 * t9348;
    let t54164 = t14011 * t9666;
    let t54166 = t14538 * t51329;
    let t54168 = t4028 * t9131;
    (t54158, t54160, t54162, t54164, t54166, t54168)
}
