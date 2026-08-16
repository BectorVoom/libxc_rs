//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1017/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1017(t36127: f64, t36141: f64, t36152: f64, t36154: f64, t41263: f64, t41265: f64, t41271: f64, t41274: f64, t41277: f64, t41279: f64, t41281: f64, t41283: f64, t41285: f64, t41287: f64, t41289: f64, t41291: f64) -> f64 {
    let t41293 = 0.15931384926072697607e-1_f64 * t41263 - 0.32452821145703643273e-2_f64 * t41265 - 0.15965655602485078086e0_f64 * t36127 - 0.10620923284048465071e-1_f64 * t36141 + 0.15965655602485078086e0_f64 * t36152 + 0.2660942600414179681e-1_f64 * t36154 - 0.10348844076463272911e-2_f64 * t41271 + 0.68186654135613354324e-2_f64 * t41274 + 0.22728884711871118108e-1_f64 * t41277 + 0.9072038638458063915e-3_f64 * t41279 + 0.45360193192290319575e-3_f64 * t41281 - 0.12700854093841289481e-2_f64 * t41283 - 0.63504270469206447405e-3_f64 * t41285 - 0.12700854093841289482e-2_f64 * t41287 - 0.63504270469206447408e-3_f64 * t41289 + 0.16934472125121719309e-2_f64 * t41291;
    t41293
}
