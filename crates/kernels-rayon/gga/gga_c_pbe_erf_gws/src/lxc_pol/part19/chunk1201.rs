//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1201/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1201(t1114: f64, t20112: f64, t1105: f64, t6854: f64, t12041: f64, t19894: f64, t3028: f64, t376: f64, t4383: f64, t9847: f64, t3916: f64, t4384: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29775 = t1114 * t20112;
    let t30104 = t6854 * t1105;
    let t34773 = t12041 * t19894;
    let t34838 = t376 * t3028;
    let t34850 = t1114 * t9847 * t4383;
    let t34922 = t3916 * t4384;
    (t29775, t30104, t34773, t34838, t34850, t34922)
}
