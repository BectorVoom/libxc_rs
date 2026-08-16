//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1497/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1497(t334: f64, t371: f64, t533: f64, t556: f64, t1184: f64, t460: f64, t1458: f64, t89: f64, t1597: f64, t343: f64, t88: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6793 = t371 * t334;
    let t6924 = 1.0_f64 / t556 / t533;
    let t7319 = t1184 * t460;
    let t7458 = t89 * t1458;
    let t7577 = t1597 * t343;
    let t7676 = t88 * t1458;
    let t8034 = t1714 * t460;
    (t6793, t6924, t7319, t7458, t7577, t7676, t8034)
}
