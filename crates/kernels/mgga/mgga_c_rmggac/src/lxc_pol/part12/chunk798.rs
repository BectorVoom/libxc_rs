//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 798/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk798<F: Float>(t36293: F, t739: F, t36247: F, t35979: F, t4044: F, t212: F, t3076: F, t672: F, t678: F, t7901: F, t7922: F, t7928: F) -> (F, F, F, F, F, F, F) {
    let t36998 = t739 * t36293;
    let t37000 = t739 * t36247;
    let t37006 = t4044 * t35979;
    let t37017 = t672 * t212 * t3076 * t678;
    let t37018 = F::new(0.14345846630704086612e-3) * t37017;
    let t37031 = F::new(0.43905552906833964735e0) * t7901;
    let t37039 = F::new(0.9931739975102829193e-4) * t7922;
    let t37041 = F::new(0.24390119833260022651e-2) * t7928;
    (t36998, t37000, t37006, t37018, t37031, t37039, t37041)
}
