//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 652/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk652<F: Float>(t3182: F, t3205: F, t3208: F, t3211: F, t3213: F, t3216: F, t3218: F, t3221: F, t3223: F, t3226: F, t3228: F, t3231: F, t3233: F, t3236: F, t3238: F, t3241: F, t693: F, t707: F) -> (F,) {
    let t3243 = t3182 * t707 / 6.0 - t693 * t3205 / 18.0 - t3208 * t707 / 48.0 + t3211 / 240.0 + t3213 * t707 / 640.0 - t3216 / 4480.0 - t3218 * t707 / 11520.0 + t3221 / 103680.0 + t3223 * t707 / 258048.0 - t3226 / 2838528.0 - t3228 * t707 / 6881280.0 + t3231 / 89456640.0 + t3233 * t707 / 0.21233664e9 - t3236 / 0.31850496e10 - t3238 * t707 / 0.74317824e10 + t3241 / 0.1263403008e12;
    (t3243,)
}
