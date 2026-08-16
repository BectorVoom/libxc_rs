//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1272/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1272(t3892: f64, t859: f64, t13792: f64, t1114: f64, t332: f64, t3747: f64, t13793: f64, t14617: f64, t53229: f64, t53571: f64, t3912: f64, t51580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56100 = t859 * t3892;
    let t56101 = t13792 * t56100;
    let t56104 = t1114 * t3747 * t332;
    let t56105 = t56104 * t13793;
    let t56107 = t53229 * t14617;
    let t56110 = t53571 * t14617;
    let t56112 = t3912 * t51580;
    (t56101, t56104, t56105, t56107, t56110, t56112)
}
