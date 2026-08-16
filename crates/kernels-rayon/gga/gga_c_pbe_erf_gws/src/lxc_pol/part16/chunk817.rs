//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 817/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk817(t6560: f64, t6562: f64, t346: f64, t6158: f64, t5: f64, t6161: f64, t337: f64, t2121: f64, t2100: f64, t274: f64, t2251: f64, t2299: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6563 = t6560 * t6562;
    let t6566 = t6158 * t346;
    let t6568 = t5 * t6161;
    let t6569 = t337 * t6568;
    let t6570 = t2121 * t6569;
    let t6573 = t274 * t2100;
    let t6578 = t2251 * t2299;
    (t6563, t6566, t6569, t6570, t6573, t6578)
}
