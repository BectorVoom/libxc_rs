//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1108/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1108<F: Float>(t1364: F, t1635: F, t2211: F, t2471: F, t36508: F, t36515: F, t37964: F, t41637: F, t41641: F, t41647: F, t41648: F, t45622: F, t47378: F, t47381: F, t47385: F, t47390: F, t47393: F, t5898: F, t6421: F, t6441: F, t699: F, t884: F, t903: F, t9530: F) -> F {
    let t49032 = F::new(0.5987120850931904282e-1) * t47378 - F::new(0.8980681276397856423e-1) * t47381 - F::new(0.7273243107798757795e0) * t41637 + F::new(0.4363945864679254677e0) * t41641 + F::new(0.2993560425465952141e-1) * t47385 + t37964 - F::new(0.23948483403727617128e0) * t1364 * t699 * t6421 - F::new(0.11974241701863808564e0) * t884 * t2211 * t45622 - F::new(0.47896966807455234256e0) * t1364 * t2471 * t1635 - F::new(0.23948483403727617128e0) * t884 * t9530 * t5898 + F::new(0.17961362552795712846e0) * t903 * t699 * t6441 - F::new(0.95793933614910468512e0) * t47390 - F::new(0.31931311204970156171e0) * t47393 - F::new(0.66211599834018861287e-4) * t36508 + t41647 - t41648 - F::new(0.66211599834018861287e-4) * t36515;
    t49032
}
