//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1024/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1024(t15573: f64, t7710: f64, t2173: f64, t7692: f64, t10466: f64, t1250: f64, t2836: f64, t3489: f64, t7696: f64, t7699: f64, t283: f64, t3049: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26714 = t15573 * t7710;
    let t26715 = t2173 * t26714;
    let t26717 = t15573 * t7692;
    let t26718 = t2173 * t26717;
    let t26728 = t10466 * t1250;
    let t26739 = t2836 * t3489;
    let t26745 = t7696 * t7699;
    let t26748 = t3049 * t283 * t990;
    (t26714, t26715, t26717, t26718, t26728, t26739, t26745, t26748)
}
