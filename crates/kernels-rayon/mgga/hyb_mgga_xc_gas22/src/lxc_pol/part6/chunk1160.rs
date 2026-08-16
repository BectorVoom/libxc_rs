//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1160/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1160(t10: f64, t6025: f64, t6466: f64, t677: f64, t1815: f64, t2026: f64, t2024: f64, t2029: f64, t2021: f64, t2053: f64, t138: f64, t2054: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20162 = t6025 * t10;
    let t20171 = t677 * t6466;
    let t20216 = t1815 * t2026;
    let t20218 = t2024 * t20216 * t2029;
    let t20225 = 1.0_f64 / t2053 / t2021;
    let t20226 = t20225 * t10;
    let t20229 = 1.0_f64 / t138 / t2054;
    (t20162, t20171, t20216, t20218, t20225, t20226, t20229)
}
