//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2041/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2041(t154: f64, t1995: f64, t205: f64, t12290: f64, t3777: f64, t12247: f64, t551: f64, t236: f64, t3792: f64, t10021: f64, t1336: f64, t1361: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40024 = t154 * t1995;
    let t40025 = t205 * t40024;
    let t40035 = t3777 * t12290;
    let t40041 = 1.0_f64 / t12247 / t551;
    let t40042 = t40041 * t236;
    let t40046 = t3792 * t3792;
    let t40059 = t1336 * t1361 * t10021;
    (t40024, t40025, t40035, t40041, t40042, t40046, t40059)
}
