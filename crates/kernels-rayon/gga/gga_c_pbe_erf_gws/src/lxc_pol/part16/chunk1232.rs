//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1232/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1232(t13780: f64, t13859: f64, t3990: f64, t9702: f64, t13917: f64, t3258: f64, t51021: f64, t51023: f64, t1114: f64, t50942: f64, t13984: f64, t3308: f64, t859: f64) -> (f64, f64, f64, f64, f64) {
    let t53212 = t13859 * t3990 * t13780 * t9702;
    let t53227 = t13917 * t51021 * t3258 * t51023;
    let t53229 = t1114 * t50942;
    let t53230 = t53229 * t13984;
    let t53233 = t859 * t3308;
    (t53212, t53227, t53229, t53230, t53233)
}
