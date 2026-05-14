//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1007/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1007<F: Float>(t5310: F, t7327: F, t2560: F, t5303: F, t2576: F, t5332: F, t17793: F, t17795: F, t17797: F, t17800: F, t17803: F, t17805: F, t17809: F, t17811: F, t17814: F, t17817: F, t17819: F, t17822: F, t17826: F, t17828: F, t17831: F) -> (F, F, F, F) {
    let t17833 = t5310 * t7327;
    let t17835 = t2560 * t5303;
    let t17837 = t2576 * t5332;
    let t17839 = -2.0 / 9.0 * t17793 - t17795 / 96.0 + t17797 / 24.0 + t17800 / 24.0 + t17803 / 108.0 - t17805 / 16.0 + t17809 / 288.0 + t17811 / 128.0 + t17814 / 27.0 + t17817 / 128.0 - t17819 / 192.0 - t17822 / 128.0 + t17826 / 256.0 - t17828 / 8.0 - t17831 / 3.0 - t17833 / 24.0 + t17835 / 24.0 - t17837 / 576.0;
    (t17833, t17835, t17837, t17839)
}
