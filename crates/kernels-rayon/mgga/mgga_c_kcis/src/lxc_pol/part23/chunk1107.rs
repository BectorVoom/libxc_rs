//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1107/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1107(t1395: f64, t5935: f64, t491: f64, t6015: f64, t7949: f64, t15808: f64, t585: f64, t1532: f64, t1928: f64, t5627: f64, t6028: f64, t7948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28587 = t1395 * t5935;
    let t28589 = t6015 * t491;
    let t28590 = t28589 * t7949;
    let t28592 = t15808 * t585;
    let t28594 = t1532 * t1928;
    let t28595 = t28594 * t7949;
    let t28597 = t6028 * t5627;
    let t28598 = t7948 * t28597;
    (t28587, t28589, t28590, t28592, t28594, t28595, t28597, t28598)
}
