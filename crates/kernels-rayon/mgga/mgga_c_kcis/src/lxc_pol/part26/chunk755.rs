//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 755/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk755(t12: f64, t3: f64, t160: f64, t2326: f64, t8581: f64, t656: f64, t8590: f64, t4620: f64, t4714: f64, t8594: f64, t8596: f64, t8598: f64) -> (f64, f64, f64, f64) {
    let t8689 = 1.0_f64/pow_3_2(t12);
    let t8690 = t8689 * t3;
    let t8691 = t8690 * t160;
    let t8693 = t2326 * t8581;
    let t8695 = t656 * t8590;
    let t8698 = -0.34523333333333333333e1_f64 * t8594 + 0.23015555555555555556e1_f64 * t8596 - 0.26851481481481481482e1_f64 * t8598 - 0.93932222222222222223e0_f64 * t4620 + 0.73355e-1_f64 * t8691 - 0.14671e0_f64 * t8693 - 0.17116166666666666667e0_f64 * t8695 - 0.36793333333333333333e0_f64 * t4714;
    (t8691, t8693, t8695, t8698)
}
