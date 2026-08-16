//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 768/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk768(t334: f64, t371: f64, t533: f64, t556: f64, t1433: f64, t71: f64, t1458: f64, t89: f64, t1597: f64, t343: f64, t88: f64, t2130: f64, rho1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6793 = t371 * t334;
    let t6924 = 1.0_f64 / t556 / t533;
    let t7445 = t71 * t1433;
    let t7458 = t89 * t1458;
    let t7577 = t1597 * t343;
    let t7676 = t88 * t1458;
    let t8025 = t2130 * rho1;
    (t6793, t6924, t7445, t7458, t7577, t7676, t8025)
}
