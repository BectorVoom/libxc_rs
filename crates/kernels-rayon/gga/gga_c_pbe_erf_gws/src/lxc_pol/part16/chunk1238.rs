//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1238/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1238(t14136: f64, t8690: f64, t2112: f64, t2306: f64, t3972: f64, t3975: f64, t9385: f64, t13780: f64, t13859: f64, t3990: f64, t8764: f64, t14733: f64, t4390: f64) -> (f64, f64, f64, f64) {
    let t53357 = t14136 * t8690;
    let t53362 = t3972 * t3975 * t9385 * t2306 * t2112;
    let t53378 = t13859 * t3990 * t13780 * t8764;
    let t53386 = t14733 * t4390;
    (t53357, t53362, t53378, t53386)
}
