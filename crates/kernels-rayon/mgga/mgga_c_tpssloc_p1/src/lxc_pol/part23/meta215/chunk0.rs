//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 860/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk860(t1553: f64, t2403: f64, t1543: f64, t2791: f64, t1597: f64, t4509: f64, t10189: f64, t344: f64, t134: f64, t2978: f64, t10213: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13642 = t2403 * t1553;
    let t13727 = t1543 * t2791;
    let t13769 = t4509 * t1597;
    let t13779 = t10189 * t344;
    let t13783 = t134 * t2978;
    let t13784 = t13783 * t344;
    let t13797 = t60 * t10213;
    (t13642, t13727, t13769, t13779, t13783, t13784, t13797)
}
