//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 778/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk778<F: Float>(t39800: F, t39808: F, t39840: F, t39842: F, t39873: F, t39899: F, t39926: F, t39970: F, t39977: F, t39997: F, t40045: F, t40062: F, t40075: F, t40084: F, t40086: F, t40088: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43139 = 0.60975299583150056624e-3 * t39800;
    let t43141 = 0.86737941314158990616e-4 * t39808;
    let t43157 = 0.49658699875514145965e-4 * t39840;
    let t43158 = 0.11918087970123395032e-3 * t39842;
    let t43169 = 0.39726959900411316772e-4 * t39873;
    let t43179 = 0.10909864661698136692e0 * t39899;
    let t43190 = 0.39726959900411316772e-4 * t39926;
    let t43204 = 0.39726959900411316772e-4 * t39970;
    let t43207 = 0.39726959900411316772e-4 * t39977;
    let t43211 = 0.3193131120497015617e0 * t39997;
    let t43241 = 0.11918087970123395032e-3 * t40045;
    let t43267 = 0.39726959900411316772e-4 * t40062;
    let t43270 = 0.49658699875514145965e-4 * t40075;
    let t43272 = 0.3842256877732895568e-2 * t40084;
    let t43273 = 0.3842256877732895568e-2 * t40086;
    let t43274 = 0.3842256877732895568e-2 * t40088;
    (t43139, t43141, t43157, t43158, t43169, t43179, t43190, t43204, t43207, t43211, t43241, t43267, t43270, t43272, t43273, t43274)
}
