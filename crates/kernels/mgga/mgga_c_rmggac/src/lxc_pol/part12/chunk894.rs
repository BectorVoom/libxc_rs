//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 894/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk894<F: Float>(t41215: F, t7603: F, t41218: F, t41221: F, t41224: F, t41227: F, t8761: F, t36127: F, t36141: F, t36152: F, t36154: F, t41263: F, t41265: F, t41271: F, t41274: F, t41277: F, t41279: F, t41281: F) -> (F,) {
    let t41283 = t7603 * t41215;
    let t41285 = t7603 * t41218;
    let t41287 = t7603 * t41221;
    let t41289 = t7603 * t41224;
    let t41291 = t8761 * t41227;
    let t41293 = 0.15931384926072697607e-1 * t41263 - 0.32452821145703643273e-2 * t41265 - 0.15965655602485078086e0 * t36127 - 0.10620923284048465071e-1 * t36141 + 0.15965655602485078086e0 * t36152 + 0.2660942600414179681e-1 * t36154 - 0.10348844076463272911e-2 * t41271 + 0.68186654135613354324e-2 * t41274 + 0.22728884711871118108e-1 * t41277 + 0.9072038638458063915e-3 * t41279 + 0.45360193192290319575e-3 * t41281 - 0.12700854093841289481e-2 * t41283 - 0.63504270469206447405e-3 * t41285 - 0.12700854093841289482e-2 * t41287 - 0.63504270469206447408e-3 * t41289 + 0.16934472125121719309e-2 * t41291;
    (t41293,)
}
