//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 197/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk197(t850: f64, t851: f64, t47: f64, t673: f64, t678: f64, t680: f64, t187: f64, t2: f64, t103: f64, t198: f64, t5: f64, t56: f64, t643: f64, t665: f64, t668: f64, t681: f64, t845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t852 = t850 * t851;
    let t858 = t47 * t673;
    let t859 = t678 * t680;
    let t864 = t187 * t2;
    let t867 = t187 * t47;
    let t870 = t643 + t665 + t187 * (0.53236443333333333332e-3_f64 * t5 * t103 * t198 + 1.0_f64 * t845 * t852 - t643 - t665 + 0.18311555036753159941e-3_f64 * t5 * t103 * t56 + 0.58482233974552040708e0_f64 * t858 * t859) - 0.18311555036753159941e-3_f64 * t864 * t668 - 0.58482233974552040708e0_f64 * t867 * t681;
    (t852, t858, t859, t864, t867, t870)
}
