//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 215/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk215(t220: f64, t617: f64, t186: f64, t616: f64, t174: f64, t205: f64, t567: f64, t213: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t618 = t220 * t617;
    let t619 = t186 * t618;
    let t621 = 4.0_f64 / 15.0_f64 * t616 * t619;
    let t623 = t174 * t567 * t205;
    let t624 = 0.18891666666666666667e-2_f64 * t623;
    let t625 = t56 * t213;
    (t618, t619, t621, t623, t624, t625)
}
