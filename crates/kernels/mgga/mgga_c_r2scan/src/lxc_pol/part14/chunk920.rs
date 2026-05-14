//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 920/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk920<F: Float>(t11687: F, t10730: F, t10732: F, t10745: F, t10770: F, t11393: F, t11399: F, t11672: F, t11676: F, t11679: F, t11681: F, t11684: F, t11700: F, t11691: F, t11694: F, t11697: F, t11703: F, t11706: F, t11709: F, t11712: F, t11715: F, t11718: F, t11721: F) -> (F, F) {
    let t12132 = 0.23115257973478049502e0 * t11687;
    let t12133 = 0.47609969197673950973e-2 * t10730 - 0.47609969197673950973e-2 * t10732 - t11393 + t10745 + t11399 + 0.32927245914677557992e0 * t11672 + 0.47609969197673950973e-2 * t10770 - 0.13099107994629972538e-1 * t11676 + 0.43663693315433241794e-2 * t11679 - 0.47609969197673950973e-2 * t11681 - 0.87327386630866483588e-2 * t11684 + t12132;
    let t12138 = 0.14282990759302185292e-1 * t11700;
    let t12146 = 0.10975748638225852664e0 * t11691 + 0.17336443480108537126e0 * t11694 + 0.47609969197673950973e-2 * t11697 + t12138 - 0.54878743191129263322e-1 * t11703 + 0.17336443480108537126e0 * t11706 + 0.2600466522016280569e0 * t11709 + 0.2600466522016280569e0 * t11712 + 0.10401866088065122276e1 * t11715 - 0.43663693315433241794e-2 * t11718 - 0.13099107994629972538e-1 * t11721;
    (t12133, t12146)
}
