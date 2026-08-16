//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1349/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1349(t14583: f64, t50998: f64, t53860: f64, t1177: f64, t1178: f64, t12099: f64, t371: f64, t1167: f64, t12275: f64, t3928: f64, t810: f64, t14831: f64, t30104: f64) -> (f64, f64, f64, f64, f64) {
    let t57755 = t50998 * t53860 * t14583;
    let t57764 = t1177 * t371 * t1178 * t12099;
    let t57779 = t12275 * t1167;
    let t57785 = t3928 * t810;
    let t57803 = t30104 * t14831;
    (t57755, t57764, t57779, t57785, t57803)
}
