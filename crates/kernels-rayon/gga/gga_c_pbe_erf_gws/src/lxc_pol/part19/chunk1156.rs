//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1156/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1156(t14881: f64, t3068: f64, t9283: f64, t1105: f64, t1206: f64, t353: f64, t4386: f64) -> (f64, f64, f64, f64, f64) {
    let t14882 = t14881 * t3068;
    let t14883 = t9283 * t14882;
    let t14886 = t1206 * t1105;
    let t14887 = t353 * t14886;
    let t14888 = t4386 * t14887;
    (t14882, t14883, t14886, t14887, t14888)
}
