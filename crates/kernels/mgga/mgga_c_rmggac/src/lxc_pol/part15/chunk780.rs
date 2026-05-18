//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 780/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk780<F: Float>(t37017: F, t7901: F, t7922: F, t7928: F, t7949: F, t8340: F, t8344: F, t8347: F, t8353: F, t8359: F, t8363: F, t8369: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37018 = F::new(0.14345846630704086612e-3) * t37017;
    let t37031 = F::new(0.43905552906833964735e0) * t7901;
    let t37039 = F::new(0.9931739975102829193e-4) * t7922;
    let t37041 = F::new(0.24390119833260022651e-2) * t7928;
    let t37047 = F::new(3.0) * t7949;
    let t38187 = F::new(0.68186654135613354322e-2) * t8340;
    let t38188 = F::new(0.72042316457491791906e-3) * t8344;
    let t38191 = F::new(0.72042316457491791906e-3) * t8347;
    let t38192 = F::new(0.72042316457491791906e-3) * t8353;
    let t38193 = F::new(0.72042316457491791906e-3) * t8359;
    let t38194 = F::new(0.72042316457491791906e-3) * t8363;
    let t38196 = F::new(0.68186654135613354322e-2) * t8369;
    (t37018, t37031, t37039, t37041, t37047, t38187, t38188, t38191, t38192, t38193, t38194, t38196)
}
