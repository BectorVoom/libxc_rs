//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1117/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1117(t28594: f64, t7949: f64, t5627: f64, t6028: f64, t7948: f64, t1548: f64, t5748: f64, t27520: f64, t6029: f64, t1552: f64, t5752: f64, t5932: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28595 = t28594 * t7949;
    let t28597 = t6028 * t5627;
    let t28598 = t7948 * t28597;
    let t28600 = t5748 * t1548;
    let t28602 = t27520 * t6029;
    let t28604 = t5752 * t1552;
    let t28606 = t7948 * t5932;
    (t28595, t28597, t28598, t28600, t28602, t28604, t28606)
}
