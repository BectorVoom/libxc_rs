//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 401/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk401(t1378: f64, t20: f64, t161: f64, t703: f64, t1369: f64, t1372: f64, t1375: f64, t696: f64, t697: f64) -> (f64, f64, f64) {
    let t1379 = t1378 * t20;
    let t1380 = t703 * t161;
    let t1383 = t1369 / 2.0_f64 + 0.627e-1_f64 * t1372 * t697 - 0.418e-1_f64 * t696 * t1375 + 0.786258e-2_f64 * t1379 * t1380;
    (t1379, t1380, t1383)
}
