//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 512/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk512<F: Float>(t235: F, t7262: F, t649: F, t876: F, t27: F, t2084: F, t352: F, t2145: F, t3924: F, t839: F, t333: F, t2139: F, t511: F, t899: F, t794: F, t321: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7263 = t235 * t7262;
    let t7264 = t649 * t876;
    let t7265 = t27 * t7264;
    let t7266 = t7263 * t7265;
    let t7267 = 0.68186654135613354322e-2 * t7266;
    let t7268 = t2084 * t352;
    let t7269 = t27 * t7268;
    let t7270 = t2145 * t7269;
    let t7273 = t235 * t3924;
    let t7274 = t649 * t839;
    let t7275 = t27 * t7274;
    let t7276 = t7273 * t7275;
    let t7277 = 0.6818665413561335432e-1 * t7276;
    let t7278 = t2084 * t333;
    let t7279 = t27 * t7278;
    let t7280 = t2139 * t7279;
    let t7282 = t899 * t511;
    let t7284 = t27 * t649 * t794;
    let t7285 = t7282 * t7284;
    let t7286 = 0.20455996240684006296e-1 * t7285;
    let t7287 = t2084 * t321;
    (t7263, t7265, t7267, t7269, t7270, t7273, t7275, t7277, t7279, t7280, t7282, t7284, t7286, t7287)
}
