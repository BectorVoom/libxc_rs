//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1462/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1462(t3242: f64, t415: f64, t61: f64, t42341: f64, t44696: f64, t42344: f64, t483: f64, t1210: f64, t1174: f64, t3561: f64, t698: f64, t11738: f64, t11739: f64, t248: f64, t3570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44827 = 1.0_f64 / t415 / t3242;
    let t44828 = t61 * t44827;
    let t44833 = t44696 * t42341;
    let t44834 = t483 * t42344;
    let t44836 = t44833 * t1210 * t44834;
    let t44847 = t1174 * t698 * t3561;
    let t44851 = t11738 * t248 * t3570 * t11739;
    (t44828, t44833, t44834, t44836, t44847, t44851)
}
