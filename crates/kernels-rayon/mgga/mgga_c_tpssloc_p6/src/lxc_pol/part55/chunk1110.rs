//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1110/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1110(t1437: f64, t8307: f64, t8513: f64, t1409: f64, t31011: f64, t1433: f64, t79: f64, t31047: f64, t7687: f64, t1983: f64, t3701: f64, t7752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33106 = t8307 * t1437;
    let t33107 = t8513 * t33106;
    let t33111 = t8513 * t31011 * t1409;
    let t33118 = t79 * t1433;
    let t33119 = t8513 * t33118;
    let t33129 = t31047 * t7687;
    let t33131 = 3.0_f64 * t1983 * t33129;
    let t33136 = t3701 * t7752;
    (t33106, t33107, t33111, t33118, t33119, t33129, t33131, t33136)
}
