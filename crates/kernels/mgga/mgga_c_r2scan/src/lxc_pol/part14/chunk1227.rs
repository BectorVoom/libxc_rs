//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1227/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1227<F: Float>(t40070: F, t40076: F, t40086: F, t38056: F, t38062: F, t38617: F, t40059: F, t40064: F, t40068: F, t40073: F, t40081: F, t40084: F) -> F {
    let t41680 = F::new(0.11902492299418487743e0) * t40070;
    let t41682 = F::new(0.95219938395347901946e-2) * t40076;
    let t41687 = F::new(0.93149212406257582492e-1) * t40086;
    let t41688 = F::new(0.17336443480108537126e0) * t40059 + F::new(0.17465477326173296718e-1) * t40064 + F::new(0.52396431978519890152e-1) * t40068 - t41680 + F::new(0.2600466522016280569e0) * t40073 + t41682 - t38617 - F::new(0.23115257973478049502e0) * t38056 + F::new(0.93149212406257582492e-1) * t38062 - F::new(0.26198215989259945076e-1) * t40081 - F::new(0.1047928639570397803e0) * t40084 - t41687;
    t41688
}
