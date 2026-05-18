//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 873/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk873<F: Float>(t275: F, t9596: F, t1347: F, t2475: F, t41828: F, t41882: F, t41884: F, t41922: F, t41960: F, t9658: F, t41977: F, t41979: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43874 = F::new(2.0) * t275 * t9596;
    let t43877 = t1347 * t2475;
    let t43878 = F::new(0.39726959900411316772e-4) * t41828;
    let t43891 = F::new(0.39726959900411316772e-4) * t41882;
    let t43892 = F::new(0.39726959900411316772e-4) * t41884;
    let t43911 = F::new(0.11918087970123395032e-3) * t41922;
    let t43937 = F::new(0.11918087970123395032e-3) * t41960;
    let t43948 = F::new(2.0) * t275 * t9658;
    let t43956 = F::new(0.3193131120497015617e0) * t41977;
    let t43957 = F::new(0.39726959900411316772e-4) * t41979;
    (t43874, t43877, t43878, t43891, t43892, t43911, t43937, t43948, t43956, t43957)
}
