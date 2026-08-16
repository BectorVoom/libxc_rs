//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 837/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk837(t2153: f64, t803: f64, t2155: f64, t314: f64, t1306: f64, t135: f64, t2149: f64, t2156: f64, t273: f64, t5483: f64, t5488: f64, t5496: f64, t5502: f64, t5504: f64, t5580: f64, t5583: f64, t5587: f64, t5753: f64, t5756: f64, t6058: f64, t805: f64) -> (f64, f64, f64) {
    let t6062 = t2153 * t803;
    let t6065 = 1.0_f64 / t2155 / t314;
    let t6069 = -3.0_f64 * t1306 * t2149 * t2156 * t803 + t135 * t273 * t6058 * t805 + 2.0_f64 * t135 * t273 * t6062 * t6065 - t5483 - t5488 - t5496 + t5502 + t5504 - t5580 - t5583 + t5587 - t5753 - t5756;
    (t6062, t6065, t6069)
}
