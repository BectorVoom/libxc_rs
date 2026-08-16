//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 780/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk780(t37017: f64, t7901: f64, t7922: f64, t7928: f64, t7949: f64, t8340: f64, t8344: f64, t8347: f64, t8353: f64, t8359: f64, t8363: f64, t8369: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37018 = 0.14345846630704086612e-3_f64 * t37017;
    let t37031 = 0.43905552906833964735e0_f64 * t7901;
    let t37039 = 0.9931739975102829193e-4_f64 * t7922;
    let t37041 = 0.24390119833260022651e-2_f64 * t7928;
    let t37047 = 3.0_f64 * t7949;
    let t38187 = 0.68186654135613354322e-2_f64 * t8340;
    let t38188 = 0.72042316457491791906e-3_f64 * t8344;
    let t38191 = 0.72042316457491791906e-3_f64 * t8347;
    let t38192 = 0.72042316457491791906e-3_f64 * t8353;
    let t38193 = 0.72042316457491791906e-3_f64 * t8359;
    let t38194 = 0.72042316457491791906e-3_f64 * t8363;
    let t38196 = 0.68186654135613354322e-2_f64 * t8369;
    (t37018, t37031, t37039, t37041, t37047, t38187, t38188, t38191, t38192, t38193, t38194, t38196)
}
