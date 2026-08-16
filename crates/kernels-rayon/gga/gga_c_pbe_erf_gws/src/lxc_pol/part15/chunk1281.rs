//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1281/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1281(t3958: f64, t6148: f64, t352: f64, t830: f64, t9292: f64, t3965: f64, t8649: f64, t14136: f64, t8700: f64, t4002: f64, t8746: f64, t14696: f64, t29287: f64, t3972: f64, t3975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53840 = t3958 * t6148;
    let t53841 = t830 * t352;
    let t53843 = t53840 * t53841 * t9292;
    let t53846 = t3965 * t8649;
    let t53848 = t14136 * t8700;
    let t53852 = t8746 * t4002;
    let t53856 = t3972 * t3975 * t29287 * t14696;
    (t53841, t53843, t53846, t53848, t53852, t53856)
}
