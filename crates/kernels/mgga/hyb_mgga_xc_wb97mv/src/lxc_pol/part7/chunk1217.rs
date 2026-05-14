//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1217/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1217<F: Float>(t3046: F, t583: F, t1190: F, t1192: F, t1194: F, t1196: F, t1198: F, t1200: F, t1202: F, t1911: F, t29411: F, t51: F, t54: F, t564: F, t57: F, t60: F, t611: F, t63: F, t66: F, t69: F, t8339: F) -> (F,) {
    let t29429 = t3046 * t3046;
    let t29446 = t3046 * t583;
    let t29463 = t611 * t29411 / 0.1263403008e12 - t564 * t29411 / 18.0 + t57 * t29429 / 320.0 - t60 * t29429 / 5760.0 + t63 * t29429 / 129024.0 - t66 * t29429 / 3440640.0 + t69 * t29429 / 0.10616832e9 - t1911 * t29429 / 0.37158912e10 + t51 * t29429 / 3.0 - t54 * t29429 / 24.0 - 8.0 / 3.0 * t1190 * t29446 + t1192 * t29446 / 2.0 - t1194 * t29446 / 20.0 + t1196 * t29446 / 288.0 - t1198 * t29446 / 5376.0 + t1200 * t29446 / 122880.0 - t1202 * t29446 / 3317760.0 + t8339 * t29446 / 103219200.0;
    (t29463,)
}
