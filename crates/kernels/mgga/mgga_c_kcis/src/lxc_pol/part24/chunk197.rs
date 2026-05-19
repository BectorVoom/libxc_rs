//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 197/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk197<F: Float>(t850: F, t851: F, t47: F, t673: F, t678: F, t680: F, t187: F, t2: F, t103: F, t198: F, t5: F, t56: F, t643: F, t665: F, t668: F, t681: F, t845: F) -> (F, F, F, F, F, F) {
    let t852 = t850 * t851;
    let t858 = t47 * t673;
    let t859 = t678 * t680;
    let t864 = t187 * t2;
    let t867 = t187 * t47;
    let t870 = t643 + t665 + t187 * (F::cast_from(0.53236443333333333332e-3_f64) * t5 * t103 * t198 + F::new(1.0) * t845 * t852 - t643 - t665 + F::cast_from(0.18311555036753159941e-3_f64) * t5 * t103 * t56 + F::cast_from(0.58482233974552040708e0_f64) * t858 * t859) - F::cast_from(0.18311555036753159941e-3_f64) * t864 * t668 - F::cast_from(0.58482233974552040708e0_f64) * t867 * t681;
    (t852, t858, t859, t864, t867, t870)
}
