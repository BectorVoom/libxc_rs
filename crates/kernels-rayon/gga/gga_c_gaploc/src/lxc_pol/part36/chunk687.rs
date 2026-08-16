//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 687/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk687(t12792: f64, t493: f64, t492: f64, t10318: f64, t2321: f64, t9074: f64, t3158: f64, t993: f64, t2268: f64, t10268: f64, t4261: f64, t2854: f64, t3085: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12793 = t493 * t12792;
    let t12794 = t492 * t12793;
    let t12797 = t10318 * t2321;
    let t12798 = t9074 * t12797;
    let t12799 = 0.23712505529730124666e-2_f64 * t12798;
    let t12800 = t3158 * t993;
    let t12802 = 0.19918504644973304719e0_f64 * t2268 * t12800;
    let t12803 = t4261 * t10268;
    let t12804 = t9074 * t12803;
    let t12805 = 0.47425011059460249332e-2_f64 * t12804;
    let t12806 = t2854 * t3085;
    (t12793, t12794, t12797, t12799, t12800, t12802, t12803, t12805, t12806)
}
