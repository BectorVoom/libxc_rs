//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1126/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1126(t16377: f64, t2030: f64, t16390: f64, t6799: f64, t138: f64, t16351: f64, t16420: f64, t6941: f64, t16326: f64, t22265: f64, t16323: f64, t6879: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48388 = t2030 * t16377;
    let t48402 = t6799 * t16390;
    let t48428 = t16351 * t138;
    let t48487 = t6941 * t16420;
    let t48526 = t22265 * t16326;
    let t48528 = t16323 * t6879;
    (t48388, t48402, t48428, t48487, t48526, t48528)
}
