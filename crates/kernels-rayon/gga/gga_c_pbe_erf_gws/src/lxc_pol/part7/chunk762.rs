//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 762/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk762(t6231: f64, t6241: f64, t858: f64, t867: f64, t6240: f64, t2157: f64, t2155: f64, t2306: f64, t346: f64, t2382: f64, t2150: f64, t2074: f64, t337: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6242 = t6231 * t6241;
    let t6244 = t867 * t858 * t6242;
    let t6246 = t6240 * t6244 / 16.0_f64;
    let t6247 = t6231 * t2157;
    let t6249 = t867 * t858 * t6247;
    let t6251 = t2155 * t6249 / 16.0_f64;
    let t6252 = t2306 * t346;
    let t6253 = t2382 * t6252;
    let t6255 = t6253 * t2150 / 16.0_f64;
    let t6257 = t337 * t5 * t2074;
    (t6242, t6244, t6246, t6247, t6249, t6251, t6252, t6253, t6255, t6257)
}
