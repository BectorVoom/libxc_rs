//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1207/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1207(t2354: f64, t859: f64, t2118: f64, t838: f64, t14138: f64, t822: f64, t2232: f64, t4386: f64, t13872: f64, t13953: f64, t13930: f64, t19906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51714 = t859 * t2354;
    let t51717 = t2118 * t838;
    let t51719 = t822 * t51717 * t14138;
    let t51721 = t4386 * t2232;
    let t51724 = t13953 * t13872;
    let t51726 = t19906 * t13930;
    (t51714, t51717, t51719, t51721, t51724, t51726)
}
