//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1031/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1031<F: Float>(t8832: F, t8837: F, t8844: F, t8846: F, t8852: F, t8856: F, t8860: F, t8864: F, t8872: F, t9597: F, t9599: F, t10376: F, t37183: F, t7780: F, t8200: F) -> (F, F, F, F, F) {
    let t42528 = F::new(0.638468998399467591e-4) * t8832;
    let t42529 = F::new(0.638468998399467591e-4) * t8837;
    let t42530 = F::new(0.212822999466489197e-4) * t8844;
    let t42531 = F::new(0.212822999466489197e-4) * t8846;
    let t42534 = F::new(0.60975299583150056624e-3) * t8852;
    let t42535 = F::new(0.60975299583150056624e-3) * t8856;
    let t42536 = F::new(0.60975299583150056624e-3) * t8860;
    let t42537 = F::new(0.60975299583150056624e-3) * t8864;
    let t42539 = F::new(0.17961362552795712846e0) * t8872;
    let t42540 = F::new(2.0) * t9597;
    let t42541 = F::new(2.0) * t9599;
    let t42542 = t42534 + t42535 + t42536 + t42537 + t10376 - F::new(0.31931311204970156171e0) * t7780 + t42539 + t42540 + t8200 + t37183 + t42541;
    (t42528, t42529, t42530, t42531, t42542)
}
