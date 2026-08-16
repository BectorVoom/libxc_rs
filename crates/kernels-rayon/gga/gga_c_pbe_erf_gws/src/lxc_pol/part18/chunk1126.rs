//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1126/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1126(t1176: f64, t923: f64, t931: f64, t3985: f64, t376: f64, t911: f64, t2210: f64, t3958: f64) -> (f64, f64, f64, f64) {
    let t14113 = t1176 * t923 * t931;
    let t14114 = t14113 * t3985;
    let t14116 = t911 * t376;
    let t14121 = t3958 * t2210;
    (t14113, t14114, t14116, t14121)
}
