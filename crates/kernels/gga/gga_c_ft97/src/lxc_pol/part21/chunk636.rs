//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 636/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk636<F: Float>(t15805: F, t401: F, t1711: F, t25: F, t371: F, t428: F, t15664: F, t1631: F, t15648: F, t7914: F, t15668: F, t3057: F, t938: F, t374: F, t1725: F, t4480: F) -> (F, F, F, F, F, F, F, F) {
    let t15806 = t15805 * t401;
    let t15810 = t1711 * t25;
    let t15811 = t371 * t15810;
    let t15812 = t15805 * t428;
    let t15819 = t1631 * t15664;
    let t15822 = t7914 * t15648;
    let t15825 = t1631 * t15668;
    let t15828 = t3057 * t938;
    let t15829 = t374 * t15828;
    let t15837 = t1725 * t4480;
    (t15806, t15811, t15812, t15819, t15822, t15825, t15829, t15837)
}
