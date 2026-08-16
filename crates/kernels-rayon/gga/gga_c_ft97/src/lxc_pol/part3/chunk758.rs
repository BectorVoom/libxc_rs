//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 758/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk758(t15664: f64, t1631: f64, t15648: f64, t7914: f64, t15668: f64, t3057: f64, t938: f64, t374: f64, t1725: f64, t4480: f64, t173: f64, t4479: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15819 = t1631 * t15664;
    let t15822 = t7914 * t15648;
    let t15825 = t1631 * t15668;
    let t15828 = t3057 * t938;
    let t15829 = t374 * t15828;
    let t15837 = t1725 * t4480;
    let t15839 = t173 * t4479;
    (t15819, t15822, t15825, t15829, t15837, t15839)
}
