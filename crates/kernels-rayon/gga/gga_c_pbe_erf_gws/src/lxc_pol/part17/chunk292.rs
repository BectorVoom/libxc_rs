//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 292/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk292(t858: f64, t875: f64, t867: f64, t866: f64, t338: f64, t348: f64, t839: f64, t331: f64, t855: f64, t863: f64) -> (f64, f64, f64, f64, f64) {
    let t876 = t858 * t875;
    let t877 = t867 * t876;
    let t879 = t866 * t877 / 96.0_f64;
    let t882 = 7.0_f64 / 288.0_f64 * t348 * t839 * t338;
    let t884 = t863 * t855 * t331;
    (t876, t877, t879, t882, t884)
}
