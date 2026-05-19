//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 913/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk913<F: Float>(t310: F, t7506: F, t7514: F, t7518: F, t22: F, t30174: F, t420: F, t56: F, t7507: F, t7513: F, t174: F, t30779: F, t7322: F) -> (F, F, F, F, F) {
    let t31388 = t310 * t7506;
    let t31389 = t31388 * t7514;
    let t31390 = F::cast_from(0.12862205435420921092e-2_f64) * t31389;
    let t31391 = t31388 * t7518;
    let t31392 = F::cast_from(0.1886885537376249124e-2_f64) * t31391;
    let t31402 = F::new(1.0) / t22 / t30174;
    let t31404 = t31402 * t56 * t420;
    let t31406 = t7507 * t31404 * t7513;
    let t31407 = F::cast_from(0.94322839859753421338e-2_f64) * t31406;
    let t31419 = t7322 * t30779 * t174;
    (t31390, t31392, t31404, t31407, t31419)
}
