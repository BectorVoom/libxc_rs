//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 630/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk630<F: Float>(t3002: F, t3029: F, t3032: F, t3035: F, t3037: F, t3040: F, t3042: F, t3045: F, t3047: F, t3050: F, t3052: F, t3055: F, t3057: F, t3060: F, t3062: F, t3065: F, t565: F, t584: F) -> (F,) {
    let t3067 = t3002 * t584 / 6.0 - t565 * t3029 / 18.0 - t3032 * t584 / 48.0 + t3035 / 240.0 + t3037 * t584 / 640.0 - t3040 / 4480.0 - t3042 * t584 / 11520.0 + t3045 / 103680.0 + t3047 * t584 / 258048.0 - t3050 / 2838528.0 - t3052 * t584 / 6881280.0 + t3055 / 89456640.0 + t3057 * t584 / 0.21233664e9 - t3060 / 0.31850496e10 - t3062 * t584 / 0.74317824e10 + t3065 / 0.1263403008e12;
    (t3067,)
}
