//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1473/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1473(t1011: f64, t1212: f64, t44706: f64, t11692: f64, t11693: f64, t11697: f64, t11853: f64, t1213: f64, t248: f64, t3570: f64, t11163: f64, t1227: f64, t3521: f64) -> (f64, f64, f64, f64) {
    let t45080 = t44706 * t1011 * t1212;
    let t45086 = t11692 * t11697 * t11693;
    let t45102 = t1213 * t248 * t3570 * t11853;
    let t45108 = t1227 * t248 * t3521 * t11163;
    (t45080, t45086, t45102, t45108)
}
