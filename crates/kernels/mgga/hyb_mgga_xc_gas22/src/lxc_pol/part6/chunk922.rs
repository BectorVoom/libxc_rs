//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 922/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk922<F: Float>(t166: F, t3188: F, t169: F, t2098: F, t151: F, t2070: F, t3165: F, t3191: F, t3196: F, t3201: F, t3206: F, t3211: F, t3216: F, t3221: F, t708: F, t8226: F, t8231: F, t8236: F, t8241: F) -> F {
    let t8246 = t166 * t3188;
    let t8251 = t169 * t3188;
    let t8256 = t2098 * t3188;
    let t8261 = t151 * t3188;
    let t8266 = -t8226 * t708 / F::new(24.0) - t3191 * t2070 / F::new(48.0) + t8231 * t708 / F::new(320.0) + t3196 * t2070 / F::new(640.0) - t8236 * t708 / F::new(5760.0) - t3201 * t2070 / F::new(11520.0) + t8241 * t708 / F::new(129024.0) + t3206 * t2070 / F::new(258048.0) - t8246 * t708 / F::new(3440640.0) - t3211 * t2070 / F::new(6881280.0) + t8251 * t708 / F::new(0.10616832e9) + t3216 * t2070 / F::new(0.21233664e9) - t8256 * t708 / F::new(0.37158912e10) - t3221 * t2070 / F::new(0.74317824e10) + t8261 * t708 / F::new(3.0) + t3165 * t2070 / F::new(6.0);
    t8266
}
