//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 636/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk636<F: Float>(t3019: F, t3046: F, t3049: F, t3052: F, t3054: F, t3057: F, t3059: F, t3062: F, t3064: F, t3067: F, t3069: F, t3072: F, t3074: F, t3077: F, t3079: F, t3082: F, t564: F, t583: F) -> (F,) {
    let t3084 = t3019 * t583 / 6.0 - t564 * t3046 / 18.0 - t3049 * t583 / 48.0 + t3052 / 240.0 + t3054 * t583 / 640.0 - t3057 / 4480.0 - t3059 * t583 / 11520.0 + t3062 / 103680.0 + t3064 * t583 / 258048.0 - t3067 / 2838528.0 - t3069 * t583 / 6881280.0 + t3072 / 89456640.0 + t3074 * t583 / 0.21233664e9 - t3077 / 0.31850496e10 - t3079 * t583 / 0.74317824e10 + t3082 / 0.1263403008e12;
    (t3084,)
}
