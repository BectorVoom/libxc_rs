//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1060/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1060(t1511: f64, t5342: f64, t1502: f64, t1506: f64, t16540: f64, t555: f64, t114: f64, t5119: f64, t557: f64, t1518: f64, t1527: f64, t1599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16554 = t1511 * t5342;
    let t16556 = t1502 * t1502;
    let t16557 = 1.0_f64 / t16556;
    let t16559 = t1506 * t1506;
    let t16560 = 1.0_f64 / t16559;
    let t16563 = 0.91082604192152556044e5_f64 * t555 * t16557 * t16540 * t16560;
    let t16565 = t5119 * t114 * t557;
    let t16569 = 36.0_f64 * t1599 * t1518 * t1527;
    (t16554, t16557, t16560, t16563, t16565, t16569)
}
