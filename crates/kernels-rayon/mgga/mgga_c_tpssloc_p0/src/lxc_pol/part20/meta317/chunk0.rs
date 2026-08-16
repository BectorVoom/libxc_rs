//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1578/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1578(t11529: f64, t1179: f64, t1174: f64, t3431: f64, t3460: f64, t3456: f64, t135: f64, t3439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11530 = t11529 * t1179;
    let t11531 = t1174 * t11530;
    let t11533 = t3431 * t3460;
    let t11534 = t1174 * t11533;
    let t11536 = t3431 * t3456;
    let t11537 = t1174 * t11536;
    let t11539 = t135 * t3439;
    (t11530, t11531, t11533, t11534, t11536, t11537, t11539)
}
