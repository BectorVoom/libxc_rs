//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1198/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1198(t4029: f64, t51407: f64, t14083: f64, t888: f64, t1189: f64, t6590: f64, t2276: f64, t2299: f64, t3969: f64, t876: f64, t9246: f64, t2134: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51408 = t51407 * t4029;
    let t51412 = t14083 * t888;
    let t51414 = t1189 * t6590;
    let t51415 = 595.0_f64 / 5184.0_f64 * t51414;
    let t51421 = t2276 * t3969 * t2299;
    let t51430 = t9246 * t876;
    let t51431 = t2134 * t51430;
    (t51408, t51412, t51415, t51421, t51430, t51431)
}
