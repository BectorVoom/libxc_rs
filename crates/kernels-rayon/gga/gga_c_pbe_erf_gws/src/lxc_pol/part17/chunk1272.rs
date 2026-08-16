//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1272/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1272(t1144: f64, t4387: f64, t859: f64, t14136: f64, t14420: f64, t19906: f64, t6683: f64, t904: f64, t14423: f64, t3989: f64, t8904: f64, t4127: f64, t4419: f64) -> (f64, f64, f64, f64) {
    let t53699 = t859 * t1144 * t4387;
    let t53700 = t14136 * t53699;
    let t53704 = 7.0_f64 / 72.0_f64 * t19906 * t14420;
    let t53710 = t904 * t6683;
    let t53713 = t3989 * t53710 * t14423 * t8904;
    let t53715 = t4127 * t4419;
    (t53700, t53704, t53713, t53715)
}
