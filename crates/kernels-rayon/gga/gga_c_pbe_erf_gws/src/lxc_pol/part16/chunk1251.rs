//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1251/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1251(t13793: f64, t53688: f64, t14657: f64, t51584: f64, t1144: f64, t4387: f64, t859: f64, t14136: f64, t6683: f64, t904: f64, t14423: f64, t3989: f64, t8904: f64) -> (f64, f64, f64, f64) {
    let t53693 = t53688 * t13793;
    let t53695 = t14657 * t51584;
    let t53699 = t859 * t1144 * t4387;
    let t53700 = t14136 * t53699;
    let t53710 = t904 * t6683;
    let t53713 = t3989 * t53710 * t14423 * t8904;
    (t53693, t53695, t53700, t53713)
}
