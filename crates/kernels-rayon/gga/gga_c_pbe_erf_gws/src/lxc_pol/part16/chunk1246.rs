//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1246/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1246(t14657: f64, t50891: f64, t1114: f64, t51916: f64, t51919: f64, t50935: f64, t13793: f64, t1112: f64, t2306: f64, t3074: f64, t833: f64, t837: f64) -> (f64, f64, f64, f64, f64) {
    let t53564 = t14657 * t50891;
    let t53566 = t1114 * t51916;
    let t53567 = t53566 * t51919;
    let t53571 = t1114 * t50935;
    let t53572 = t53571 * t13793;
    let t53577 = t3074 * t2306 * t1112 * t837 * t833;
    (t53564, t53567, t53571, t53572, t53577)
}
