//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1158/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1158(t12213: f64, t2409: f64, t4016: f64, t4182: f64, t938: f64, t3067: f64, t3111: f64, t3950: f64, t850: f64, t833: f64, t1123: f64, t13815: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14664 = t2409 * t12213 * t4016;
    let t14667 = t4182 * t938;
    let t14669 = t2409 * t3067 * t14667;
    let t14673 = t850 * t3111 * t3950;
    let t14674 = t14673 * t833;
    let t14677 = t850 * t1123 * t13815;
    (t14664, t14667, t14669, t14673, t14674, t14677)
}
