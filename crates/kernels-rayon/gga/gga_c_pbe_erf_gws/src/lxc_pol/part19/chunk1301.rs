//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1301/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1301(t12204: f64, t3989: f64, t3990: f64, t53283: f64, t2409: f64, t39460: f64, t3965: f64, t3972: f64, t54499: f64, t54590: f64, t8884: f64, t13984: f64, t56104: f64) -> (f64, f64, f64, f64) {
    let t56840 = t3989 * t3990 * t53283 * t12204;
    let t56843 = t3965 * t2409 * t39460;
    let t56847 = t3972 * t54499 * t8884 * t54590;
    let t56849 = t56104 * t13984;
    (t56840, t56843, t56847, t56849)
}
