//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1112/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1112(t14058: f64, t935: f64, t2327: f64, t4049: f64, t4021: f64, t885: f64, t2149: f64) -> (f64, f64, f64, f64, f64) {
    let t14059 = t14058 * t935;
    let t14060 = 7.0_f64 / 288.0_f64 * t14059;
    let t14061 = t4049 * t2327;
    let t14063 = t4021 * t885;
    let t14064 = t14063 * t2149;
    (t14059, t14060, t14061, t14063, t14064)
}
