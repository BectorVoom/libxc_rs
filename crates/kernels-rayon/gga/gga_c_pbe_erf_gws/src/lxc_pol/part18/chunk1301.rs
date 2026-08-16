//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1301/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1301(t1134: f64, t13796: f64, t13859: f64, t3097: f64, t1113: f64, t3972: f64, t3975: f64, t814: f64, t9847: f64, t3222: f64, t3721: f64, t51548: f64, param_a_c: f64) -> (f64, f64, f64) {
    let t56604 = t13859 * t13796 * t3097 * t1134;
    let t56613 = t3972 * t3975 * t1113 * t9847 * t814;
    let t56618 = t3972 * t51548 * t3721 * param_a_c * t3222;
    (t56604, t56613, t56618)
}
