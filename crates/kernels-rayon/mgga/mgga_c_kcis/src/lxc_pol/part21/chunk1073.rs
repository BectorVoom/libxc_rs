//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1073/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1073(t26710: f64, t7718: f64, t2842: f64, t15573: f64, t7710: f64, t2173: f64, t7692: f64) -> (f64, f64, f64, f64, f64) {
    let t26711 = t7718 * t26710;
    let t26712 = t2842 * t26711;
    let t26714 = t15573 * t7710;
    let t26715 = t2173 * t26714;
    let t26717 = t15573 * t7692;
    (t26711, t26712, t26714, t26715, t26717)
}
