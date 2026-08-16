//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 145/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk145(t43: f64, t234: f64, t269: f64, t271: f64, t277: f64, t340: f64, t355: f64, t364: f64, t95: f64, t50: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t367 = -t234 + t269 + t271 + 0.25844881434903430496e-2_f64 * t95 * t277 * t340 + t355 * t364 / 2.0_f64;
    let t368 = piecewise3(t44, zeta_threshold, t43);
    let t370 = 1.0_f64 / t50;
    let t371 = pow_1_3(t370);
    (t367, t368, t370, t371)
}
