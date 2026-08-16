//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1127/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1127(t274: f64, t837: f64, t850: f64, t851: f64, t833: f64, t3955: f64, t894: f64, t2118: f64, t332: f64, t353: f64, t4387: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14125 = t274 * t837;
    let t14127 = t850 * t851 * t14125;
    let t14128 = t14127 * t833;
    let t14130 = t3955 * t894;
    let t14135 = t2118 * t332;
    let t14138 = t859 * t353 * t4387;
    (t14125, t14127, t14128, t14130, t14135, t14138)
}
