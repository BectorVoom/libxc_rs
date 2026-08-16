//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1123/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1123(t14257: f64, t14298: f64, t14332: f64, t14358: f64, t2053: f64, t4116: f64, t944: f64, t1211: f64, t6854: f64, t2051: f64, t2423: f64, t4120: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14360 = t14257 + t14298 + t14332 + t14358;
    let t14364 = t4116 * t2053;
    let t14365 = t14364 * t944;
    let t14368 = t1211 * t6854;
    let t14369 = t14368 * t2051;
    let t14372 = t4120 * t2423;
    (t14360, t14364, t14365, t14368, t14369, t14372)
}
