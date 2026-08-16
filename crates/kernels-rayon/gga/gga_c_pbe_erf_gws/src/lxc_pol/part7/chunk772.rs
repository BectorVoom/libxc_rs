//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 772/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk772(t2119: f64, t2382: f64, t5: f64, t745: f64, t337: f64, t2121: f64, t810: f64, t816: f64, t2084: f64, t3257: f64, t2083: f64, t2112: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6339 = t2382 * t2119;
    let t6340 = t5 * t745;
    let t6341 = t337 * t6340;
    let t6342 = t2121 * t6341;
    let t6344 = t6339 * t6342 / 32.0_f64;
    let t6345 = t816 * t810;
    let t6347 = t3257 * t2084 * t6345;
    let t6350 = t2112 * t2083;
    (t6339, t6340, t6341, t6342, t6344, t6345, t6347, t6350)
}
