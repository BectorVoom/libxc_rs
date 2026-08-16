//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1246/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1246(t53260: f64, t1162: f64, t13796: f64, t2190: f64, t3989: f64, t3952: f64, t8751: f64, t14423: f64, t14682: f64, t2158: f64, t14617: f64, t50943: f64) -> (f64, f64, f64, f64, f64) {
    let t53261 = 7.0_f64 / 144.0_f64 * t53260;
    let t53264 = t3989 * t13796 * t1162 * t2190;
    let t53266 = t3952 * t8751;
    let t53270 = t3989 * t14682 * t14423 * t2158;
    let t53272 = t50943 * t14617;
    (t53261, t53264, t53266, t53270, t53272)
}
