//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1285/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1285(t14682: f64, t3140: f64, t3989: f64, t56296: f64, t11753: f64, t3972: f64, t3975: f64, t11990: f64, t13776: f64, t53236: f64, t14733: f64, t53699: f64) -> (f64, f64, f64, f64) {
    let t56299 = t3989 * t14682 * t56296 * t3140;
    let t56302 = t3972 * t3975 * t11753;
    let t56305 = t13776 * t53236 * t11990;
    let t56307 = t14733 * t53699;
    (t56299, t56302, t56305, t56307)
}
