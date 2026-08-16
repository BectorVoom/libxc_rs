//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1180/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1180(t10469: f64, t1603: f64, t11058: f64, t11045: f64, t11064: f64, t1597: f64, t43052: f64, t1553: f64, t9709: f64, t13797: f64, t13783: f64, t1599: f64, t2402: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47840 = t1603 * t10469;
    let t47841 = t47840 * t11058;
    let t47853 = t47840 * t11045;
    let t47857 = t47840 * t11064;
    let t48019 = t43052 * t1597;
    let t48103 = t9709 * t1553;
    let t48221 = t13797 * t1597;
    let t48279 = t13783 * t1597;
    let t48336 = t973 * t2402 * t1599;
    (t47840, t47841, t47853, t47857, t48019, t48103, t48221, t48279, t48336)
}
