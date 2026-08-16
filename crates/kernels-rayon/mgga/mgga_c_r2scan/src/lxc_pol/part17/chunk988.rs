//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 988/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk988(t11700: f64, t11691: f64, t11694: f64, t11697: f64, t11703: f64, t11706: f64, t11709: f64, t11712: f64, t11715: f64, t11718: f64, t11721: f64, t11753: f64) -> (f64, f64) {
    let t12138 = 0.14282990759302185292e-1_f64 * t11700;
    let t12146 = 0.10975748638225852664e0_f64 * t11691 + 0.17336443480108537126e0_f64 * t11694 + 0.47609969197673950973e-2_f64 * t11697 + t12138 - 0.54878743191129263322e-1_f64 * t11703 + 0.17336443480108537126e0_f64 * t11706 + 0.2600466522016280569e0_f64 * t11709 + 0.2600466522016280569e0_f64 * t11712 + 0.10401866088065122276e1_f64 * t11715 - 0.43663693315433241794e-2_f64 * t11718 - 0.13099107994629972538e-1_f64 * t11721;
    let t12158 = 0.19514881078765566037e-1_f64 * t11753;
    (t12146, t12158)
}
