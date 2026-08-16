//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 669/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk669(t1883: f64, t6547: f64, t131: f64, t209: f64, t229: f64, t1878: f64) -> (f64, f64, f64) {
    let t6548 = t6547 * t1883;
    let t6549 = 0.19190897446562641759e-1_f64 * t6548;
    let t6551 = t229 * t131 * t209;
    let t6552 = t1878 * t6551;
    (t6549, t6551, t6552)
}
