//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 154/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk154(t27: f64, t13: f64, t14: f64, t1: f64, t119: f64, t155: f64, t156: f64, t4: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t387 = t27 * t27;
    let t388 = 1.0_f64 / t387;
    let t389 = t13 * t388;
    let t390 = 1.0_f64 / t14;
    let t391 = t390 * t1;
    let t392 = t119 * t155;
    let t393 = t391 * t392;
    let t395 = t4 * t156;
    (t387, t388, t389, t390, t391, t392, t393, t395)
}
