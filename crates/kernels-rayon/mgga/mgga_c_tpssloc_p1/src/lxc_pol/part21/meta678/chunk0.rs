//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2485/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2485(t12858: f64, t2535: f64, t12606: f64, t707: f64, t751: f64, t4205: f64, t9868: f64, t193: f64, t776: f64, t3966: f64, t4194: f64, t607: f64, t750: f64) -> (f64, f64, f64, f64, f64) {
    let t46310 = t12858 * t2535;
    let t46317 = t707 * t751 * t12606;
    let t46335 = t4205 * t9868;
    let t46341 = t193 * t776;
    let t46348 = t4194 * t750 * t3966 * t607;
    (t46310, t46317, t46335, t46341, t46348)
}
