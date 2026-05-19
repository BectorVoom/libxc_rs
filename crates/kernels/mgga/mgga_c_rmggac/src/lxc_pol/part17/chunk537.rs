//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 537/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk537<F: Float>(t235: F, t7262: F, t2084: F, t352: F, t27: F, t2145: F, t3924: F, t333: F, t2139: F, t511: F, t899: F, t321: F) -> (F, F, F, F, F, F, F, F) {
    let t7263 = t235 * t7262;
    let t7268 = t2084 * t352;
    let t7269 = t27 * t7268;
    let t7270 = t2145 * t7269;
    let t7271 = F::cast_from(0.18183107769496894486e-1_f64) * t7270;
    let t7273 = t235 * t3924;
    let t7278 = t2084 * t333;
    let t7279 = t27 * t7278;
    let t7280 = t2139 * t7279;
    let t7282 = t899 * t511;
    let t7287 = t2084 * t321;
    (t7263, t7269, t7271, t7273, t7279, t7280, t7282, t7287)
}
