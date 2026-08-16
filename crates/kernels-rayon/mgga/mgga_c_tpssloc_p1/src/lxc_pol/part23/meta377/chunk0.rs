//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1179/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1179(t4166: f64, t9666: f64, t9973: f64, t10024: f64, t1500: f64, t9670: f64, t9600: f64, t1540: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46881 = t4166 * t9666;
    let t46957 = t4166 * t9973;
    let t47047 = t1500 * t10024;
    let t47092 = t4166 * t9670;
    let t47275 = t4166 * t9600;
    let t47787 = t9698 * t1540;
    (t46881, t46957, t47047, t47092, t47275, t47787)
}
