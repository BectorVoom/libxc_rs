//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 109/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk109(t261: f64, t93: f64, t108: f64, t260: f64, t1: f64, t183: f64, t22: f64) -> (f64, f64, f64) {
    let t262 = t93 * t261;
    let t265 = (t260 / 2.0_f64 + t262 / 2.0_f64) * t108;
    let t266 = t183 * t1;
    let t267 = t266 * t22;
    (t265, t266, t267)
}
