//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1197/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1197(t10621: f64, t164: f64, t600: f64, t10655: f64, t5257: f64, t10651: f64, t16399: f64, t10558: f64, t1702: f64, t10562: f64, t16369: f64, t1020: f64, t2639: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29248 = t10621 * t600 * t164;
    let t29252 = t5257 * t10655;
    let t29254 = t16399 * t10651;
    let t29262 = t1702 * t10558;
    let t29264 = t16369 * t10562;
    let t29279 = t1020 * t2639;
    (t29248, t29252, t29254, t29262, t29264, t29279)
}
