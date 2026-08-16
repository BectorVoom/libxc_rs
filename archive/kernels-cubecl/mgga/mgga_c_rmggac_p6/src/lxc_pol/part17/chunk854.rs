//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 854/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk854<F: Float>(t42204: F, t16156: F, t9190: F, t9184: F, t36920: F, t7933: F, t9081: F, t303: F, t577: F, t7934: F, t357: F, t132: F, t1412: F) -> (F, F, F, F, F, F, F) {
    let t42205 = F::cast_from(0.17877131955185092547e-3_f64) * t42204;
    let t42206 = t16156 * t9190;
    let t42207 = F::cast_from(0.11918087970123395031e-3_f64) * t42206;
    let t42217 = t16156 * t9184;
    let t42234 = t7933 * t36920 * t9081;
    let t42238 = t7933 * t7934 * t577 * t303;
    let t42239 = F::cast_from(0.72042316457491791906e-3_f64) * t42238;
    let t42242 = t7933 * t7934 * t577 * t357;
    let t42243 = F::cast_from(0.72042316457491791906e-3_f64) * t42242;
    let t42246 = t7933 * t7934 * t1412 * t132;
    (t42205, t42207, t42217, t42234, t42239, t42243, t42246)
}
