//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 809/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk809(t1: f64, t6383: f64, t2313: f64, t814: f64, t2156: f64, t274: f64, t343: f64, t6201: f64, t915: f64, t2250: f64, t2259: f64, t6269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6384 = t6383 * t1;
    let t6390 = t2313 * t814;
    let t6395 = t274 * t2156;
    let t6396 = t6395 * t343;
    let t6401 = t6201 * t915;
    let t6402 = t2250 * t6401;
    let t6403 = t6402 * t2259;
    let t6409 = t6269 * t343;
    (t6384, t6390, t6395, t6396, t6401, t6402, t6403, t6409)
}
