//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 861/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk861(t13797: f64, t344: f64, t135: f64, t340: f64, t10189: f64, t1597: f64, t10224: f64, t1592: f64, t973: f64, t1599: f64, t698: f64, t10508: f64, t1616: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13798 = t13797 * t344;
    let t13822 = t135 * t340;
    let t13847 = t10189 * t1597;
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    let t13908 = t698 * t1599;
    let t13909 = t973 * t13908;
    let t13965 = t248 * t10508 * t1616;
    (t13798, t13822, t13847, t13896, t13909, t13965)
}
