//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 526/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk526(t2323: f64, t935: f64, t2074: f64, t904: f64, t933: f64, t1331: f64, t22: f64) -> (f64, f64, f64) {
    let t2324 = t2323 * t935;
    let t2327 = t933 * t904 * t2074;
    let t2331 = 1.0_f64 / t22 / t1331;
    (t2324, t2327, t2331)
}
