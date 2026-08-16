//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 789/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk789(t2285: f64, t2289: f64, t6232: f64, t858: f64, t867: f64, t866: f64, t3205: f64, t336: f64, t2182: f64, t343: f64, t2135: f64, t2168: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6517 = t2289 * t2285;
    let t6520 = t867 * t858 * t6232;
    let t6522 = t866 * t6520 / 96.0_f64;
    let t6523 = t3205 * t336;
    let t6524 = t343 * t2182;
    let t6526 = t6523 * t2135 * t6524;
    let t6528 = 3.0_f64 / 16.0_f64 * t2168 * t6526;
    (t6517, t6520, t6522, t6523, t6524, t6526, t6528)
}
