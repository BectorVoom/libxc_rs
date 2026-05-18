//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 866/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk866<F: Float>(t39698: F, t39701: F, t39785: F, t39796: F, t39800: F, t39808: F, t39840: F, t39842: F, t39873: F, t39899: F, t39926: F, t39970: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43107 = F::new(0.10909864661698136692e0) * t39698;
    let t43108 = F::new(0.47896966807455234256e0) * t39701;
    let t43135 = F::new(0.60975299583150056624e-3) * t39785;
    let t43138 = F::new(0.60975299583150056624e-3) * t39796;
    let t43139 = F::new(0.60975299583150056624e-3) * t39800;
    let t43141 = F::new(0.86737941314158990616e-4) * t39808;
    let t43157 = F::new(0.49658699875514145965e-4) * t39840;
    let t43158 = F::new(0.11918087970123395032e-3) * t39842;
    let t43169 = F::new(0.39726959900411316772e-4) * t39873;
    let t43179 = F::new(0.10909864661698136692e0) * t39899;
    let t43190 = F::new(0.39726959900411316772e-4) * t39926;
    let t43204 = F::new(0.39726959900411316772e-4) * t39970;
    (t43107, t43108, t43135, t43138, t43139, t43141, t43157, t43158, t43169, t43179, t43190, t43204)
}
