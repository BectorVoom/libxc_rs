//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1197/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1197(t1477: f64, t2153: f64, t863: f64, t2160: f64, t328: f64, t6552: f64, t331: f64, t20934: f64, t858: f64, t867: f64, t21287: f64, t6240: f64) -> (f64, f64, f64) {
    let t21293 = t863 * t2153 * t1477;
    let t21294 = t21293 * t2160;
    let t21295 = 35.0_f64 / 36.0_f64 * t21294;
    let t21296 = t6552 * t328;
    let t21298 = t863 * t21296 * t331;
    let t21302 = t21298 * t867 * t858 * t20934 / 4.0_f64;
    let t21306 = 3.0_f64 / 8.0_f64 * t6240 * t867 * t858 * t21287;
    (t21295, t21302, t21306)
}
