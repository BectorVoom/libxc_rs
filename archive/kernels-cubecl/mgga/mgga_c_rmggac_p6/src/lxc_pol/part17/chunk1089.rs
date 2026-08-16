//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1089/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1089<F: Float>(t117: F, t33235: F, t2295: F, t31057: F, t46391: F, t6355: F, t9000: F, t498: F, t511: F, t7230: F, t7231: F, t9969: F) -> (F, F, F, F) {
    let t47830 = t33235 * t117;
    let t47831 = t47830 * t2295;
    let t47833 = t31057 * t46391;
    let t47835 = t6355 * t9000;
    let t47840 = t7230 * t7231 * t511 * t9969 * t498;
    (t47831, t47833, t47835, t47840)
}
