//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1214/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1214(t14881: f64, t2417: f64, t353: f64, t859: f64, t4111: f64, t4386: f64, t810: f64, t14186: f64, t892: f64, t14188: f64, t19906: f64, t14274: f64, t9270: f64) -> (f64, f64, f64, f64, f64) {
    let t52381 = t859 * t353 * t14881 * t2417;
    let t52393 = t4386 * t353 * t4111 * t810;
    let t52417 = t859 * t892 * t14186;
    let t52432 = t19906 * t14188;
    let t52473 = t9270 * t14274;
    (t52381, t52393, t52417, t52432, t52473)
}
