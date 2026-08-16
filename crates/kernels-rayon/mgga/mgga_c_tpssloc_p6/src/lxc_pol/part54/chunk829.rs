//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 829/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk829(t1268: f64, t1458: f64, t2039: f64, t4028: f64, t7042: f64, t7676: f64, t7787: f64, t7801: f64, t7170: f64, t7687: f64, t1807: f64, t2085: f64) -> (f64, f64, f64) {
    let t7900 = 2.0_f64 * t1268 * t7801 + 2.0_f64 * t1458 * t7042 + 2.0_f64 * t2039 * t4028 + 2.0_f64 * t2039 * t7676 + t7787;
    let t7904 = t7170 * t7687;
    let t7910 = t1807 * t2085;
    (t7900, t7904, t7910)
}
