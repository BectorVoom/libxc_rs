//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1238/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1238(t14733: f64, t4484: f64, t1112: f64, t361: f64, t51543: f64, t13917: f64, t9388: f64, t1178: f64, t13783: f64, t8787: f64, t13925: f64, t22493: f64) -> (f64, f64, f64, f64) {
    let t53134 = t14733 * t4484;
    let t53138 = t361 * t51543 * t1112;
    let t53140 = t13917 * t53138 * t9388;
    let t53152 = t13917 * t1178 * t8787 * t13783;
    let t53155 = 7.0_f64 / 144.0_f64 * t22493 * t13925;
    (t53134, t53140, t53152, t53155)
}
