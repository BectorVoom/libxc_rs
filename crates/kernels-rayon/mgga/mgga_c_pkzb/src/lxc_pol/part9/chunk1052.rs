//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1052/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1052(t16388: f64, t1708: f64, t2590: f64, t5256: f64, t5247: f64, t1717: f64, t5295: f64, t1723: f64, t173: f64, t5286: f64, t588: f64, t603: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16389 = t16388 * t1708;
    let t16399 = t2590 * t5256;
    let t16400 = t16399 * t5247;
    let t16402 = t1717 * t5295;
    let t16403 = t16402 * t1723;
    let t16405 = t5286 * t173;
    let t16406 = t588 * t16405;
    let t16407 = t16406 * t603;
    (t16389, t16399, t16400, t16403, t16405, t16406, t16407)
}
