//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1454/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1454(t2113: f64, t4072: f64, t119931: f64, t2108: f64, t2240: f64, t131: f64, t27331: f64, t46104: f64, t8662: f64, t12571: f64, t31867: f64, t33676: f64, t9231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122920 = t2113 * t4072;
    let t122941 = t2240 * t119931 * t2108;
    let t122945 = t2240 * t27331 * t131;
    let t122952 = t46104 * t8662;
    let t122955 = t12571 * t31867;
    let t122960 = t9231 * t33676;
    (t122920, t122941, t122945, t122952, t122955, t122960)
}
