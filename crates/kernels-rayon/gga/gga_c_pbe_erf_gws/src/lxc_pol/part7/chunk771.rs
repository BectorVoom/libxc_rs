//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 771/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk771(t337: f64, t6326: f64, t2121: f64, t6325: f64, t2365: f64, t885: f64, t2149: f64, t2146: f64, t346: f64, t4395: f64, t2382: f64, t2124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6327 = t337 * t6326;
    let t6328 = t2121 * t6327;
    let t6330 = t6325 * t6328 / 32.0_f64;
    let t6331 = t2365 * t885;
    let t6332 = t6331 * t2149;
    let t6333 = t2146 * t6332;
    let t6334 = 7.0_f64 / 24.0_f64 * t6333;
    let t6335 = t4395 * t346;
    let t6336 = t2382 * t6335;
    let t6338 = t6336 * t2124 / 32.0_f64;
    (t6327, t6328, t6330, t6331, t6332, t6334, t6335, t6336, t6338)
}
