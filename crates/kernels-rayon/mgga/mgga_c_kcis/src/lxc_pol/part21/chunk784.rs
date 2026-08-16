//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 784/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk784(t5: f64, t9077: f64, t4: f64, t691: f64, t789: f64, t128: f64, t2438: f64, t8556: f64, t144: f64, t717: f64, t728: f64, t2459: f64) -> (f64, f64, f64, f64, f64) {
    let t9078 = t5 * t9077;
    let t9082 = t789 * t4 * t691;
    let t9088 = t128 * t2438;
    let t9089 = t9088 * t8556;
    let t9092 = t144 * t128;
    let t9093 = t717 * t728;
    let t9094 = t9093 * t2459;
    (t9078, t9082, t9089, t9092, t9094)
}
