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
    let t8266 = -t8226 * t708 / F::cast_from(24.0_f64) - t3191 * t2070 / F::cast_from(48.0_f64) + t8231 * t708 / F::cast_from(320.0_f64) + t3196 * t2070 / F::cast_from(640.0_f64) - t8236 * t708 / F::cast_from(5760.0_f64) - t3201 * t2070 / F::cast_from(11520.0_f64) + t8241 * t708 / F::cast_from(129024.0_f64) + t3206 * t2070 / F::cast_from(258048.0_f64) - t8246 * t708 / F::cast_from(3440640.0_f64) - t3211 * t2070 / F::cast_from(6881280.0_f64) + t8251 * t708 / F::cast_from(0.10616832e9_f64) + t3216 * t2070 / F::cast_from(0.21233664e9_f64) - t8256 * t708 / F::cast_from(0.37158912e10_f64) - t3221 * t2070 / F::cast_from(0.74317824e10_f64) + t8261 * t708 / F::cast_from(3.0_f64) + t3165 * t2070 / F::cast_from(6.0_f64);
    t8266
}
