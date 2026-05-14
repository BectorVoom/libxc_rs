//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 931/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk931<F: Float>(t143: F, t8683: F, t160: F, t3205: F, t163: F, t166: F, t169: F, t2081: F, t3218: F, t3223: F, t3228: F, t3233: F, t693: F, t707: F, t711: F, t715: F, t719: F, t723: F, t727: F, t731: F, t735: F) -> (F, F) {
    let t145 = 0.135e1 < t143;
    let t8684 = piecewise3(t145, t8683, 0.0);
    let t8701 = t160 * t3205;
    let t8706 = t163 * t3205;
    let t8711 = t166 * t3205;
    let t8716 = t169 * t3205;
    let t8721 = -t693 * t8684 / 18.0 + t711 * t8684 / 240.0 - t715 * t8684 / 4480.0 + t719 * t8684 / 103680.0 - t723 * t8684 / 2838528.0 + t727 * t8684 / 89456640.0 - t731 * t8684 / 0.31850496e10 + t735 * t8684 / 0.1263403008e12 - t8701 * t707 / 5760.0 - t3218 * t2081 / 11520.0 + t8706 * t707 / 129024.0 + t3223 * t2081 / 258048.0 - t8711 * t707 / 3440640.0 - t3228 * t2081 / 6881280.0 + t8716 * t707 / 0.10616832e9 + t3233 * t2081 / 0.21233664e9;
    (t8684, t8721)
}
