//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1266/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1266<F: Float>(t100678: F, t100680: F, t100683: F, t100686: F, t100688: F, t100691: F, t28137: F, t28204: F, t29094: F, t92761: F, t97312: F, t97332: F, t97344: F) -> F {
    let t100695 = -F::cast_from(0.92835860883789062501e-5_f64) * t92761 * t29094 - F::cast_from(0.17411041666666666666e-2_f64) * t100678 + t97312 + F::cast_from(0.15476481481481481481e-2_f64) * t100680 - F::cast_from(0.34822083333333333332e-2_f64) * t100683 - F::cast_from(0.46429444444444444443e-2_f64) * t100686 + F::cast_from(0.77382407407407407407e-3_f64) * t100688 + t97332 + t97344 + F::cast_from(0.38691203703703703703e-2_f64) * t100691 - F::cast_from(0.2782641015625e-3_f64) * t28204 * t28137;
    t100695
}
