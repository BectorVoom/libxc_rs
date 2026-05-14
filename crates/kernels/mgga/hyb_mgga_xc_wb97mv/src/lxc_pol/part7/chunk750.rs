//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 750/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk750<F: Float>(t151: F, t3975: F, t3994: F, t3997: F, t3999: F, t4001: F, t4003: F, t4005: F, t4007: F, t4009: F, t4011: F, t4013: F, t4015: F, t4017: F, t4019: F, t4021: F, t4023: F, t693: F) -> (F,) {
    let t4025 = t151 * t3975 / 6.0 - t693 * t3994 / 18.0 - t3997 / 48.0 + t3999 / 240.0 + t4001 / 640.0 - t4003 / 4480.0 - t4005 / 11520.0 + t4007 / 103680.0 + t4009 / 258048.0 - t4011 / 2838528.0 - t4013 / 6881280.0 + t4015 / 89456640.0 + t4017 / 0.21233664e9 - t4019 / 0.31850496e10 - t4021 / 0.74317824e10 + t4023 / 0.1263403008e12;
    (t4025,)
}
