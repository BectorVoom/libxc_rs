//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 180/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk180(t130: f64, t139: f64, t145: f64, t459: f64, t464: f64, t458: f64, t129: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t860 = t130 * t139;
    let t862 = t860 * t145 * t459;
    let t864 = t464 * t130;
    let t866 = t139 * t145 * t458;
    let t867 = t864 * t866;
    let t869 = 3.0_f64 / 128.0_f64 * t862 - t867 / 128.0_f64;
    let t871 = 1.0_f64 / t129;
    (t860, t862, t864, t866, t867, t869, t871)
}
