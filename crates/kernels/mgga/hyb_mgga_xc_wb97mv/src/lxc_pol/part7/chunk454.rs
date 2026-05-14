//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 454/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk454<F: Float>(t151: F, t154: F, t157: F, t160: F, t163: F, t166: F, t169: F, t2053: F, t2081: F, t2109: F, t693: F, t711: F, t715: F, t719: F, t723: F, t727: F, t731: F, t735: F) -> (F,) {
    let t2114 = t151 * t2053 / 6.0 - t693 * t2081 / 18.0 - t154 * t2053 / 48.0 + t711 * t2081 / 240.0 + t157 * t2053 / 640.0 - t715 * t2081 / 4480.0 - t160 * t2053 / 11520.0 + t719 * t2081 / 103680.0 + t163 * t2053 / 258048.0 - t723 * t2081 / 2838528.0 - t166 * t2053 / 6881280.0 + t727 * t2081 / 89456640.0 + t169 * t2053 / 0.21233664e9 - t731 * t2081 / 0.31850496e10 - t2109 * t2053 / 0.74317824e10 + t735 * t2081 / 0.1263403008e12;
    (t2114,)
}
