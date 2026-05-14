//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1238/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1238<F: Float>(t10743: F, t2053: F, t2109: F, t22001: F, t3238: F, t3975: F, t3994: F, t3997: F, t3999: F, t4001: F, t4003: F, t4005: F, t4007: F, t4009: F, t4011: F, t4013: F, t4015: F, t4017: F, t4019: F, t4021: F, t4023: F, t6507: F, t707: F, t8684: F) -> (F,) {
    let t30242 = t6507 * t3994 * t2053 / 412876800.0 + 10.0 / 3.0 * t3997 * t2053 - 2.0 / 3.0 * t3999 * t2053 - 7.0 / 8.0 * t4001 * t2053 + t4003 * t2053 / 8.0 + 9.0 / 80.0 * t4005 * t2053 - t4007 * t2053 / 80.0 - 11.0 / 1152.0 * t4009 * t2053 + t4011 * t2053 / 1152.0 + 13.0 / 21504.0 * t4013 * t2053 - t4015 * t2053 / 21504.0 - t4017 * t2053 / 32768.0 + t4019 * t2053 / 491520.0 + 17.0 / 13271040.0 * t4021 * t2053 - t4023 * t2053 / 13271040.0 - 19.0 / 412876800.0 * t22001 * t3975 * t2053 - t2109 * t10743 * t707 / 0.37158912e10 - t3238 * t8684 / 0.37158912e10;
    (t30242,)
}
