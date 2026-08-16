//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1250/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1250(t13917: f64, t13919: f64, t9347: f64, t9603: f64, t13792: f64, t8602: f64, t14767: f64, t2379: f64, t13791: f64, t3039: f64, t13984: f64, t14657: f64, t51714: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53668 = t13917 * t13919 * t9347;
    let t53671 = t13917 * t13919 * t9603;
    let t53675 = t13792 * t8602;
    let t53677 = t14767 * t2379;
    let t53688 = t3039 * t13791;
    let t53689 = t53688 * t13984;
    let t53691 = t14657 * t51714;
    (t53668, t53671, t53675, t53677, t53688, t53689, t53691)
}
