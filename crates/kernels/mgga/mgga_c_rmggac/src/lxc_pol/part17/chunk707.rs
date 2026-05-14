//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 707/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk707<F: Float>(t271: F, t3899: F, t638: F, t641: F, t1347: F, t2128: F, t212: F, t3076: F, t672: F, t678: F, t7901: F, t7922: F, t7928: F, t7949: F, t8340: F, t8344: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36983 = t638 * t3899 * t271 * t641;
    let t36984 = 0.69557008413371175709e-2 * t36983;
    let t36992 = t1347 * t2128;
    let t37017 = t672 * t212 * t3076 * t678;
    let t37018 = 0.14345846630704086612e-3 * t37017;
    let t37031 = 0.43905552906833964735e0 * t7901;
    let t37039 = 0.9931739975102829193e-4 * t7922;
    let t37041 = 0.24390119833260022651e-2 * t7928;
    let t37047 = 3.0 * t7949;
    let t38187 = 0.68186654135613354322e-2 * t8340;
    let t38188 = 0.72042316457491791906e-3 * t8344;
    (t36984, t36992, t37018, t37031, t37039, t37041, t37047, t38187, t38188)
}
