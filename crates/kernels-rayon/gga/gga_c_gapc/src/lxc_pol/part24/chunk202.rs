//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 202/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk202(t255: f64, t256: f64, t62: f64, t1: f64, t252: f64, t348: f64, t352: f64, t354: f64, t14: f64, t351: f64) -> (f64, f64, f64, f64, f64) {
    let t737 = 1.0_f64 / t256 / t255;
    let t738 = t62 * t737;
    let t740 = t348 * t252 * t1;
    let t745 = -0.14921166666666666667e-3_f64 * t352 - 0.39332083333333333333e-2_f64 * t354;
    let t748 = -t740 * t351 / 12.0_f64 + t14 * t745 / 2.0_f64;
    (t737, t738, t740, t745, t748)
}
