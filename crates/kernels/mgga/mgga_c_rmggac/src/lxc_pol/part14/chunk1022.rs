//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1022/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1022<F: Float>(t2103: F, t41048: F, t41032: F, t36166: F, t36157: F, t36158: F, t36160: F, t36168: F, t36174: F, t41336: F, t41338: F, t41341: F, t41342: F, t41344: F, t41348: F, t41349: F, t41351: F) -> F {
    let t41353 = t2103 * t41048;
    let t41355 = t2103 * t41032;
    let t41358 = F::new(0.19513579069703984327e0) * t36166;
    let t41360 = -F::new(0.11974241701863808564e0) * t41336 - F::new(0.15965655602485078085e0) * t41338 - t41341 + F::new(0.3193131120497015617e0) * t41342 - F::new(0.11974241701863808564e0) * t41344 + t36157 + F::new(0.2660942600414179681e-1) * t36158 - t41348 + F::new(0.9072038638458063915e-4) * t41349 + F::new(0.34093327067806677162e-2) * t41351 - F::new(0.45457769423742236216e-2) * t41353 - F::new(0.44447596769881297634e-1) * t41355 - F::new(0.39914139006212695215e-1) * t36160 - t41358 + F::new(0.29270368604555976491e0) * t36168 - t36174;
    t41360
}
