//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1072/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1072(t10: f64, t128: f64, t19083: f64, t395: f64, t485: f64, t5821: f64, t125: f64, t19247: f64, t4516: f64, t5833: f64, t501: f64, t5826: f64) -> (f64, f64, f64, f64) {
    let t19307 = t10 * t128 * t19083;
    let t19311 = t485 * t5821 * t395;
    let t19312 = 0.116921e2_f64 * t19311;
    let t19316 = 0.16322666666666666667e0_f64 * t125 * t4516 * t5833 * t19247;
    let t19318 = t501 * t5826 * t395;
    (t19307, t19312, t19316, t19318)
}
