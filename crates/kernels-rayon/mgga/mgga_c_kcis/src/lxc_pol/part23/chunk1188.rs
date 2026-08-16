//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1188/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1188(t18210: f64, t27415: f64, t7898: f64, t2237: f64, t11425: f64, t1386: f64, t94491: f64, t94469: f64, t1466: f64, t4109: f64, t3245: f64, t7932: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94496 = t18210 * t27415;
    let t94497 = t7898 * t94496;
    let t94499 = t2237 * t94496;
    let t94519 = t1386 * t11425;
    let t94524 = t7898 * t94491;
    let t94526 = t7898 * t94469;
    let t94528 = t4109 * t1466;
    let t94537 = t3245 * t7932;
    (t94497, t94499, t94519, t94524, t94526, t94528, t94537)
}
