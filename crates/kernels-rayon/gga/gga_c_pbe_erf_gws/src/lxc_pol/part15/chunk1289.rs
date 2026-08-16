//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1289/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1289(t3965: f64, t8736: f64, t14784: f64, t50994: f64, t20091: f64, t4157: f64, t14637: f64, t3974: f64, t3990: f64, t8804: f64, t2409: f64, t26647: f64, t3959: f64) -> (f64, f64, f64, f64, f64) {
    let t53950 = t3965 * t8736;
    let t53952 = t50994 * t14784;
    let t53953 = 7.0_f64 / 288.0_f64 * t53952;
    let t53959 = t20091 * t4157;
    let t53963 = t14637 * t3990 * t3974 * t8804;
    let t53966 = t3959 * t2409 * t26647;
    (t53950, t53953, t53959, t53963, t53966)
}
