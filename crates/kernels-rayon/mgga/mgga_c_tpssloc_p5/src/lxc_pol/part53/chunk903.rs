//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 903/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk903(t1516: f64, t8343: f64, t12571: f64, t8301: f64, t1437: f64, t8307: f64, t8513: f64, t1409: f64, t31011: f64, t1433: f64, t79: f64, t4028: f64, t8326: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32847 = t8343 * t1516;
    let t33103 = t12571 * t8301;
    let t33106 = t8307 * t1437;
    let t33107 = t8513 * t33106;
    let t33111 = t8513 * t31011 * t1409;
    let t33118 = t79 * t1433;
    let t33119 = t8513 * t33118;
    let t33151 = t4028 * t8326;
    (t32847, t33103, t33106, t33107, t33111, t33118, t33119, t33151)
}
