//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1077/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1077(t19384: f64, t127: f64, t1563: f64, t16423: f64, t19083: f64, t19268: f64, t19349: f64, t19351: f64, t19355: f64, t19357: f64, t19359: f64, t19362: f64, t19365: f64, t19367: f64, t19373: f64, t19381: f64, t506: f64) -> (f64, f64) {
    let t19385 = 0.77947333333333333333e1_f64 * t19384;
    let t19386 = -t19349 + t19351 + t19355 + t19357 + t19359 + t19362 + t19365 + 0.1762848e3_f64 * t127 * t19367 * t19268 - t19373 - 0.146904e1_f64 * t127 * t506 * t19083 + 0.1762848e2_f64 * t127 * t1563 * t16423 - 6.0_f64 * t19381 + t19385;
    (t19385, t19386)
}
