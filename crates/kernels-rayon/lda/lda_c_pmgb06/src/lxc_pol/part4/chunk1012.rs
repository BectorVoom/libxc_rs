//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1012/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1012(t4080: f64, t4111: f64, t1180: f64, t209: f64, t211: f64, t4088: f64, t591: f64, t4094: f64, t4096: f64, t4103: f64, t574: f64, t581: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9412 = (2e-21_f64 as f64) * t4080 * t4111;
    let t9417 = 56.0_f64 / 243.0_f64 * t209 * t211 * t1180;
    let t9418 = t4088 * t591;
    let t9422 = t4094 * t591;
    let t9424 = t4096 * t4111;
    let t9426 = t574 * t4103;
    let t9429 = 32.0_f64 / 81.0_f64 * t581 * t4103;
    (t9412, t9417, t9418, t9422, t9424, t9426, t9429)
}
