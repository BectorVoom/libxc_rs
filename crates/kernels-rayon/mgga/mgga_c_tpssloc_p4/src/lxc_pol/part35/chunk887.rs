//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 887/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk887(t1553: f64, t2403: f64, t1543: f64, t2791: f64, t1597: f64, t4509: f64, t10189: f64, t10224: f64, t1592: f64, t973: f64, t1599: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13642 = t2403 * t1553;
    let t13727 = t1543 * t2791;
    let t13769 = t4509 * t1597;
    let t13847 = t10189 * t1597;
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    let t13908 = t698 * t1599;
    (t13642, t13727, t13769, t13847, t13896, t13908)
}
