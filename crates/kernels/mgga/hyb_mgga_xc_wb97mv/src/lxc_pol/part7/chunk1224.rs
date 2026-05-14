//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1224/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1224<F: Float>(t10214: F, t1861: F, t1224: F, t2987: F, t2991: F, t8154: F, t8141: F, t10215: F, t8155: F, t10224: F, t8160: F, t10223: F, t10231: F, t25540: F, t1877: F, t10219: F, t25444: F, t25446: F, t2988: F, t2989: F, t3129: F, t3854: F, t544: F, t667: F, t8126: F, t8131: F, t8135: F, t8140: F, t8142: F, t8143: F) -> (F, F) {
    let t29666 = t10214 * t1861;
    let t29672 = t2987 * t8154 * t1224 * t2991;
    let t29684 = t8141 * t1224;
    let t29692 = t2987 * t8155 * t10215;
    let t29703 = t8160 * t8155 * t10224;
    let t29705 = t10223 * t1861;
    let t29716 = t2987 * t25540 * t10231;
    let t29718 = t10223 * t1877;
    let t29728 = -7.0 / 144.0 * t8140 * t8142 * t29666 - t29672 / 36.0 - t2987 * t2988 * t3129 * t2991 / 12.0 - t2987 * t10219 * t8131 / 12.0 - t2987 * t10219 * t8135 / 24.0 - 7.0 / 72.0 * t8140 * t29684 * t8143 + t8160 * t8126 * t10224 / 8.0 - t29692 / 72.0 - t2987 * t8126 * t10215 / 24.0 - t2987 * t2989 * t667 * t3854 * t544 / 24.0 + t29703 / 24.0 - t8140 * t2989 * t29705 / 4.0 + t8160 * t2989 * t29666 / 16.0 + t8160 * t10219 * t8143 / 8.0 + 7.0 / 36.0 * t29716 - 7.0 / 144.0 * t8140 * t8142 * t29718 - 35.0 / 216.0 * t25444 * t25446 * t29705 + t8160 * t2989 * t29718 / 16.0;
    (t29705, t29728)
}
