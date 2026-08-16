//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 942/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk942(t1036: f64, t4617: f64, t10422: f64, t4574: f64, t3070: f64, t1597: f64, t4509: f64, t10189: f64, t344: f64, t4343: f64, t2986: f64, t134: f64, t2978: f64) -> (f64, f64, f64, f64, f64) {
    let t13758 = t4617 * t1036 / 2304.0_f64;
    let t13765 = t10422 * t4574;
    let t13767 = t3070 * t13765 / 3456.0_f64;
    let t13769 = t4509 * t1597;
    let t13779 = t10189 * t344;
    let t13780 = t13779 * t4343;
    let t13782 = 0.37037037037037037036e-3_f64 * t2986 * t13780;
    let t13783 = t134 * t2978;
    (t13758, t13767, t13769, t13782, t13783)
}
