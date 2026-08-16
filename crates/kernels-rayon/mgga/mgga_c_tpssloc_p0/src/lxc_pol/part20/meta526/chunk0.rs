//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2060/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2060(t205: f64, t40024: f64, t12247: f64, t551: f64, t236: f64, t3792: f64, t12283: f64, t12422: f64, t12339: f64, t3876: f64, t10021: f64, t1336: f64, t1361: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40025 = t205 * t40024;
    let t40041 = 1.0_f64 / t12247 / t551;
    let t40042 = t40041 * t236;
    let t40046 = t3792 * t3792;
    let t40052 = t12283 * t12422;
    let t40054 = t12339 * t3876;
    let t40059 = t1336 * t1361 * t10021;
    (t40025, t40041, t40042, t40046, t40052, t40054, t40059)
}
