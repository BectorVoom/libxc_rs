//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2213/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2213(t28267: f64, t81651: f64, t82074: f64, t1888: f64, t23270: f64, t25044: f64, t4300: f64, t5527: f64, t857: f64, t25038: f64, t865: f64, t23035: f64, t23237: f64, t28298: f64) -> (f64, f64, f64, f64) {
    let t98213 = t81651 * t82074 * t28267;
    let t98222 = t1888 * t23270 * t25044 * t4300;
    let t98224 = t857 * t5527;
    let t98227 = t25038 * t23270 * t98224 * t865;
    let t98234 = t23035 * t23237 * t28298;
    (t98213, t98222, t98227, t98234)
}
