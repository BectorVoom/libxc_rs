//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 424/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk424<F: Float>(t1842: F, t1883: F, t1911: F, t51: F, t54: F, t564: F, t57: F, t587: F, t591: F, t595: F, t599: F, t60: F, t603: F, t607: F, t611: F, t63: F, t66: F, t69: F) -> (F,) {
    let t1916 = t51 * t1842 / 6.0 - t564 * t1883 / 18.0 - t54 * t1842 / 48.0 + t587 * t1883 / 240.0 + t57 * t1842 / 640.0 - t591 * t1883 / 4480.0 - t60 * t1842 / 11520.0 + t595 * t1883 / 103680.0 + t63 * t1842 / 258048.0 - t599 * t1883 / 2838528.0 - t66 * t1842 / 6881280.0 + t603 * t1883 / 89456640.0 + t69 * t1842 / 0.21233664e9 - t607 * t1883 / 0.31850496e10 - t1911 * t1842 / 0.74317824e10 + t611 * t1883 / 0.1263403008e12;
    (t1916,)
}
