//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1234/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1234(t15573: f64, t29093: f64, t7788: f64, t1092: f64, t28991: f64, t92701: f64, t18513: f64, t2842: f64, t7718: f64, t29103: f64, t3500: f64, t19160: f64, t26760: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100129 = t15573 * t29093;
    let t100130 = t7788 * t100129;
    let t100133 = t1092 * t92701 * t28991;
    let t100136 = t2842 * t7718 * t18513;
    let t100139 = t7788 * t3500 * t29103;
    let t100142 = t2842 * t26760 * t19160;
    (t100129, t100130, t100133, t100136, t100139, t100142)
}
