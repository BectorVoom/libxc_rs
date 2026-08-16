//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1076/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1076(t102: f64, t128: f64, t16423: f64, t505: f64, t97: f64, t120: f64, t19083: f64, t156: f64, t496: f64, t5744: f64, t5772: f64, t19344: f64) -> (f64, f64, f64, f64, f64) {
    let t19365 = 0.1753815e2_f64 * t102 * t128 * t16423;
    let t19367 = 1.0_f64 / t505 / t97;
    let t19373 = 0.2923025e1_f64 * t102 * t120 * t19083;
    let t19381 = t496 * t156 * t5744;
    let t19383 = t5772 * t120;
    let t19384 = t19383 * t19344;
    (t19365, t19367, t19373, t19381, t19384)
}
