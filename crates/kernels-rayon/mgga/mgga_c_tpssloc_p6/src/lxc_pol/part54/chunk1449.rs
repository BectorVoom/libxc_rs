//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1449/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1449(t120952: f64, t2039: f64, t102344: f64, t1873: f64, t27188: f64, t6534: f64, t121004: f64, t121007: f64, t33234: f64, t23938: f64, t7467: f64, t26977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t122730 = t120952 * t2039;
    let t122731 = t102344 * t1873;
    let t122734 = t27188 * t6534;
    let t122735 = t121004 * t1873;
    let t122736 = t121007 * t1873;
    let t122737 = t33234 * t6534;
    let t122738 = t23938 * t7467;
    let t122739 = t26977 * t7467;
    (t122730, t122731, t122734, t122735, t122736, t122737, t122738, t122739)
}
