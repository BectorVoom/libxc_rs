//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 898/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk898<F: Float>(t36157: F, t36158: F, t36160: F, t36168: F, t36174: F, t41336: F, t41338: F, t41341: F, t41342: F, t41344: F, t41348: F, t41349: F, t41351: F, t41353: F, t41355: F, t41358: F) -> (F,) {
    let t41360 = -0.11974241701863808564e0 * t41336 - 0.15965655602485078085e0 * t41338 - t41341 + 0.3193131120497015617e0 * t41342 - 0.11974241701863808564e0 * t41344 + t36157 + 0.2660942600414179681e-1 * t36158 - t41348 + 0.9072038638458063915e-4 * t41349 + 0.34093327067806677162e-2 * t41351 - 0.45457769423742236216e-2 * t41353 - 0.44447596769881297634e-1 * t41355 - 0.39914139006212695215e-1 * t36160 - t41358 + 0.29270368604555976491e0 * t36168 - t36174;
    (t41360,)
}
