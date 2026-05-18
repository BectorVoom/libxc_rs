//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1211/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1211<F: Float>(t39601: F, t39607: F, t37702: F, t37707: F, t37714: F, t39599: F, t39604: F, t39610: F, t39616: F, t39619: F, t39622: F, t39627: F) -> (F, F) {
    let t41464 = F::new(0.10975748638225852664e-1) * t39601;
    let t41466 = F::new(0.93149212406257582492e-1) * t39607;
    let t41471 = -F::new(0.19514881078765566037e-1) * t37702 - F::new(0.90044238659382329742e0) * t37707 - F::new(0.95219938395347901946e-2) * t37714 + F::new(0.43663693315433241794e-2) * t39599 + t41464 - F::new(0.87327386630866483588e-2) * t39604 - t41466 - F::new(0.17336443480108537126e0) * t39610 - F::new(0.43902994552903410656e0) * t39616 - F::new(0.34672886960217074252e0) * t39619 - F::new(0.10401866088065122276e1) * t39622;
    let t41474 = F::new(0.46230515946956099004e0) * t39627;
    (t41471, t41474)
}
