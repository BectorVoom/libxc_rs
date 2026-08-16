//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1174/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1174(t26654: f64, t938: f64, t1161: f64, t19631: f64, t2182: f64, t2501: f64, t831: f64, t8574: f64, t2376: f64, t9688: f64, t810: f64, t8749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26655 = t26654 * t938;
    let t26668 = t19631 * t1161;
    let t26768 = t2501 * t2182;
    let t26880 = t831 * t8574;
    let t26885 = t2376 * t9688;
    let t26933 = t8749 * t810;
    (t26655, t26668, t26768, t26880, t26885, t26933)
}
