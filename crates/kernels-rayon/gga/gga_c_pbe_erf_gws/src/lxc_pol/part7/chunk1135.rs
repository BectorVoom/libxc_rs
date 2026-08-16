//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1135/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1135(t2121: f64, t3074: f64, t337: f64, t6326: f64, t6335: f64, t814: f64, t6192: f64, t6203: f64, t6253: f64, t6258: f64, t2105: f64, t810: f64) -> (f64, f64, f64, f64) {
    let t20366 = 7.0_f64 / 48.0_f64 * t3074 * t6335 * t2121 * t337 * t6326 * t814;
    let t20367 = t6203 * t6192;
    let t20370 = t6253 * t6258 / 8.0_f64;
    let t20371 = t2105 * t810;
    (t20366, t20367, t20370, t20371)
}
