//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 763/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk763<F: Float>(t42180: F, t2139: F, t27: F, t3118: F, t558: F, t40975: F, t7192: F, t16156: F, t9194: F, t9190: F, t9184: F, t36920: F, t7933: F, t9081: F, t303: F, t577: F, t7934: F) -> (F, F, F, F, F, F, F, F) {
    let t42181 = 0.19863479950205658386e-4 * t42180;
    let t42196 = t2139 * t27 * t3118 * t558;
    let t42201 = t7192 * t40975;
    let t42204 = t16156 * t9194;
    let t42205 = 0.17877131955185092547e-3 * t42204;
    let t42206 = t16156 * t9190;
    let t42207 = 0.11918087970123395031e-3 * t42206;
    let t42217 = t16156 * t9184;
    let t42234 = t7933 * t36920 * t9081;
    let t42238 = t7933 * t7934 * t577 * t303;
    (t42181, t42196, t42201, t42205, t42207, t42217, t42234, t42238)
}
