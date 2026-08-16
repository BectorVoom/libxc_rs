//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1149/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1149(t14504: f64, t14527: f64, t14553: f64, t14574: f64, t898: f64, t338: f64, t353: f64, t1161: f64, t3222: f64, t13781: f64, t3972: f64, t1113: f64, t9520: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14576 = t14504 + t14527 + t14553 + t14574;
    let t14577 = t898 * t14576;
    let t14579 = t338 * t353 * t14577;
    let t14582 = t1161 * param_a_c;
    let t14583 = t14582 * t3222;
    let t14584 = t13781 * t14583;
    let t14585 = t3972 * t14584;
    let t14587 = t1113 * t9520;
    (t14576, t14577, t14579, t14583, t14584, t14585, t14587)
}
