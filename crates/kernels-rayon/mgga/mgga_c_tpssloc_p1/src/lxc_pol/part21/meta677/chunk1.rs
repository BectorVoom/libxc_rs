//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2484/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2484(t12908: f64, t12924: f64, t4101: f64, t9912: f64, t1409: f64, t2516: f64, t4194: f64, t607: f64, t4199: f64, t9722: f64, t12887: f64, t172: f64, t763: f64) -> (f64, f64, f64, f64, f64) {
    let t46283 = t12908 * t12924;
    let t46285 = t9912 * t4101;
    let t46291 = t4194 * t2516 * t1409 * t607;
    let t46302 = t4199 * t9722;
    let t46308 = t12887 * t172 * t763;
    (t46283, t46285, t46291, t46302, t46308)
}
