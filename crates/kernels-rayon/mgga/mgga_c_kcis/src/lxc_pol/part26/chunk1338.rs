//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1338/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1338(t3738: f64, t7329: f64, t6029: f64, t97681: f64, t491: f64, t7381: f64, t7953: f64, t21971: f64, t4261: f64, t7952: f64, t5748: f64, t6034: f64) -> (f64, f64, f64, f64, f64) {
    let t102914 = t3738 * t7329;
    let t102916 = t97681 * t6029;
    let t102918 = t7381 * t491;
    let t102919 = t102918 * t7953;
    let t102922 = t7952 * t4261 * t21971;
    let t102924 = t5748 * t6034;
    (t102914, t102916, t102919, t102922, t102924)
}
