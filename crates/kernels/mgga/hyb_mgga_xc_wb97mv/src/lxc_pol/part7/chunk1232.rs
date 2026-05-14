//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1232/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1232<F: Float>(t10550: F, t8543: F, t8548: F, t10621: F, t214: F, t10549: F, t2013: F, t10559: F, t3155: F, t10564: F, t10554: F, t10558: F, t10593: F, t10874: F, t2035: F, t2038: F, t2039: F, t26109: F, t26111: F, t26114: F, t26116: F, t26161: F, t26163: F, t26198: F, t26205: F, t29991: F, t30006: F, t3157: F, t674: F, t683: F, t686: F, t8440: F, t8519: F, t8528: F, t8530: F) -> (F, F) {
    let t30019 = t8548 * t8543 * t10550;
    let t30021 = t214 * t10621;
    let t30035 = t10549 * t2013;
    let t30043 = t3155 * t8543 * t10559;
    let t30053 = t3155 * t8543 * t10564;
    let t30069 = -t8528 * t3157 * t29991 / 4.0 + t30019 / 24.0 - t3155 * t3157 * t30021 * t674 / 24.0 + t8548 * t8519 * t10550 / 8.0 - 7.0 / 72.0 * t8528 * t26198 * t10550 + t3155 * t26205 * t10554 / 6.0 - 7.0 / 144.0 * t8528 * t8530 * t30035 - 35.0 / 216.0 * t26161 * t26163 * t29991 - t30043 / 36.0 - t3155 * t3157 * t10558 * t2013 / 24.0 - 7.0 / 72.0 * t8528 * t8530 * t30006 - t30053 / 72.0 - t683 * t686 * t10593 * t2013 / 32.0 - t2035 * t2038 * t10593 * t2039 / 24.0 - t26109 / 32.0 - t26111 / 16.0 + t26114 / 24.0 + t26116 / 24.0 + 3.0 / 16.0 * t8440 * t10874;
    (t30035, t30069)
}
