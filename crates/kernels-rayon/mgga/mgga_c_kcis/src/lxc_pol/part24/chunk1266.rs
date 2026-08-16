//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1266/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1266(t100678: f64, t100680: f64, t100683: f64, t100686: f64, t100688: f64, t100691: f64, t28137: f64, t28204: f64, t29094: f64, t92761: f64, t97312: f64, t97332: f64, t97344: f64) -> f64 {
    let t100695 = -0.92835860883789062501e-5_f64 * t92761 * t29094 - 0.17411041666666666666e-2_f64 * t100678 + t97312 + 0.15476481481481481481e-2_f64 * t100680 - 0.34822083333333333332e-2_f64 * t100683 - 0.46429444444444444443e-2_f64 * t100686 + 0.77382407407407407407e-3_f64 * t100688 + t97332 + t97344 + 0.38691203703703703703e-2_f64 * t100691 - 0.2782641015625e-3_f64 * t28204 * t28137;
    t100695
}
