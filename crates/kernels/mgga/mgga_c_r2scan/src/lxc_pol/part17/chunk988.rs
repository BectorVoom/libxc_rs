//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 988/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk988<F: Float>(t11700: F, t11691: F, t11694: F, t11697: F, t11703: F, t11706: F, t11709: F, t11712: F, t11715: F, t11718: F, t11721: F, t11753: F) -> (F, F) {
    let t12138 = F::cast_from(0.14282990759302185292e-1_f64) * t11700;
    let t12146 = F::cast_from(0.10975748638225852664e0_f64) * t11691 + F::cast_from(0.17336443480108537126e0_f64) * t11694 + F::cast_from(0.47609969197673950973e-2_f64) * t11697 + t12138 - F::cast_from(0.54878743191129263322e-1_f64) * t11703 + F::cast_from(0.17336443480108537126e0_f64) * t11706 + F::cast_from(0.2600466522016280569e0_f64) * t11709 + F::cast_from(0.2600466522016280569e0_f64) * t11712 + F::cast_from(0.10401866088065122276e1_f64) * t11715 - F::cast_from(0.43663693315433241794e-2_f64) * t11718 - F::cast_from(0.13099107994629972538e-1_f64) * t11721;
    let t12158 = F::cast_from(0.19514881078765566037e-1_f64) * t11753;
    (t12146, t12158)
}
