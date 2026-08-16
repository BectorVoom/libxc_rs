//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1027/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1027(t2271: f64, t3861: f64, t905: f64, t11339: f64, t823: f64, t850: f64, t852: f64, t860: f64, t3824: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11505 = t3861 * t2271;
    let t11506 = t905 * t11505;
    let t11509 = t11339 * t823;
    let t11511 = t850 * t11509 * t852;
    let t11513 = t11511 * t860 / 96.0_f64;
    let t11514 = t6 * t3824;
    (t11505, t11506, t11509, t11511, t11513, t11514)
}
