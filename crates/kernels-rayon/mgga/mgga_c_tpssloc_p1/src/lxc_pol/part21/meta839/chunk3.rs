//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3004/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3004(t62729: f64, t62730: f64, t62732: f64, t62733: f64, t62736: f64, t62737: f64, t62739: f64, t62754: f64, t17152: f64, t42972: f64, t973: f64, t10876: f64, t13969: f64, t17983: f64) -> (f64, f64, f64) {
    let t62757 = t62729 + t62730 + t62732 + t62733 + t62736 + t62737 + t62739 + t62754;
    let t62766 = t973 * t42972 * t17152;
    let t62778 = t10876 * t13969 * t17983;
    (t62757, t62766, t62778)
}
