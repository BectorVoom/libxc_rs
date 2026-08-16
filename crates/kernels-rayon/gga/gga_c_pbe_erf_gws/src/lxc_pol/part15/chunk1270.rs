//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1270/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1270(t1118: f64, t13859: f64, t14682: f64, t2158: f64, t51530: f64, t13917: f64, t13919: f64, t9347: f64, t9603: f64, t13792: f64, t8602: f64, t14767: f64, t2379: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53664 = t13859 * t14682 * t1118 * t2158;
    let t53666 = 119.0_f64 / 1728.0_f64 * t51530;
    let t53668 = t13917 * t13919 * t9347;
    let t53671 = t13917 * t13919 * t9603;
    let t53675 = t13792 * t8602;
    let t53677 = t14767 * t2379;
    (t53664, t53666, t53668, t53671, t53675, t53677)
}
