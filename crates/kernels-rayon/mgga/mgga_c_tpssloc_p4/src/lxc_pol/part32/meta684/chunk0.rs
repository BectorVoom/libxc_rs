//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2125/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2125(t1864: f64, t5445: f64, t2240: f64, t5399: f64, t3953: f64, t3961: f64, t3967: f64, t1437: f64, t4017: f64, t72: f64, t1433: f64, t4021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96469 = t1864 * t5445;
    let t96473 = t2240 * t5399;
    let t96479 = t3953 * t3961;
    let t96482 = t3953 * t3967;
    let t96502 = t72 * t4017 * t1437;
    let t96506 = t72 * t1433 * t4021;
    (t96469, t96473, t96479, t96482, t96502, t96506)
}
