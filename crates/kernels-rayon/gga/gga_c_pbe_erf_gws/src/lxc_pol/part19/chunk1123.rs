//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1123/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1123(t14185: f64, t938: f64, t353: f64, t859: f64, t1206: f64, t814: f64) -> (f64, f64, f64, f64) {
    let t14186 = t14185 * t938;
    let t14187 = t353 * t14186;
    let t14188 = t859 * t14187;
    let t14191 = t1206 * t814;
    let t14192 = t353 * t14191;
    let t14193 = t859 * t14192;
    (t14186, t14188, t14191, t14193)
}
