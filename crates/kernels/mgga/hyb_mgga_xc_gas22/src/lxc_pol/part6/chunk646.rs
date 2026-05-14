//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 646/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk646<F: Float>(t3165: F, t3188: F, t3191: F, t3194: F, t3196: F, t3199: F, t3201: F, t3204: F, t3206: F, t3209: F, t3211: F, t3214: F, t3216: F, t3219: F, t3221: F, t3224: F, t694: F, t708: F) -> (F,) {
    let t3226 = t3165 * t708 / 6.0 - t694 * t3188 / 18.0 - t3191 * t708 / 48.0 + t3194 / 240.0 + t3196 * t708 / 640.0 - t3199 / 4480.0 - t3201 * t708 / 11520.0 + t3204 / 103680.0 + t3206 * t708 / 258048.0 - t3209 / 2838528.0 - t3211 * t708 / 6881280.0 + t3214 / 89456640.0 + t3216 * t708 / 0.21233664e9 - t3219 / 0.31850496e10 - t3221 * t708 / 0.74317824e10 + t3224 / 0.1263403008e12;
    (t3226,)
}
