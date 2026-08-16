//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 358/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk358(t1580: f64, t951: f64, t1545: f64, t1559: f64, t1561: f64, t1569: f64, t1574: f64, t300: f64, t311: f64, t924: f64, t943: f64, t942: f64) -> (f64, f64, f64, f64) {
    let t1581 = t1580 * t951;
    let t1585 = t300 * (-0.310907e-1_f64 * t1561 * t311 + 1.0_f64 * t924 * t1569 + t1545 - t1559 - 0.19751673498613801407e-1_f64 * t1574 + 0.5848223622634646207e0_f64 * t943 * t1581);
    let t1587 = 0.19751673498613801407e-1_f64 * t300 * t1574;
    let t1589 = t942 * t1580 * t951;
    (t1581, t1585, t1587, t1589)
}
