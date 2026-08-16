//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1114/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1114(t6517: f64, t7468: f64, t12571: f64, t8301: f64, t1437: f64, t8307: f64, t8513: f64, t1409: f64, t31011: f64, t7440: f64, t1433: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33101 = t6517 * t7468;
    let t33103 = t12571 * t8301;
    let t33106 = t8307 * t1437;
    let t33107 = t8513 * t33106;
    let t33111 = t8513 * t31011 * t1409;
    let t33114 = t8307 * t7440;
    let t33115 = t8513 * t33114;
    let t33118 = t79 * t1433;
    (t33101, t33103, t33106, t33107, t33111, t33114, t33115, t33118)
}
