//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1017/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1017<F: Float>(t36127: F, t36141: F, t36152: F, t36154: F, t41263: F, t41265: F, t41271: F, t41274: F, t41277: F, t41279: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F, t41291: F) -> F {
    let t41293 = F::new(0.15931384926072697607e-1) * t41263 - F::new(0.32452821145703643273e-2) * t41265 - F::new(0.15965655602485078086e0) * t36127 - F::new(0.10620923284048465071e-1) * t36141 + F::new(0.15965655602485078086e0) * t36152 + F::new(0.2660942600414179681e-1) * t36154 - F::new(0.10348844076463272911e-2) * t41271 + F::new(0.68186654135613354324e-2) * t41274 + F::new(0.22728884711871118108e-1) * t41277 + F::new(0.9072038638458063915e-3) * t41279 + F::new(0.45360193192290319575e-3) * t41281 - F::new(0.12700854093841289481e-2) * t41283 - F::new(0.63504270469206447405e-3) * t41285 - F::new(0.12700854093841289482e-2) * t41287 - F::new(0.63504270469206447408e-3) * t41289 + F::new(0.16934472125121719309e-2) * t41291;
    t41293
}
