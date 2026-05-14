//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1229/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1229<F: Float>(t3: F, t674: F, t3158: F, t10617: F, t2025: F, t683: F, t10609: F, t10583: F, t8605: F, t10843: F, t214: F, t10579: F, t10621: F, t1232: F, t2035: F, t2038: F, t2044: F, t25633: F, t25636: F, t25657: F, t25660: F, t25666: F, t25672: F, t25675: F, t25696: F, t3167: F, t3979: F, t3988: F, t6701: F, t686: F, t8536: F, t8548: F, t8562: F, t8580: F) -> (F, F, F) {
    let t29908 = t3 * t674;
    let t29909 = t3158 * t29908;
    let t29914 = t683 * t2025 * t10617;
    let t29917 = t683 * t2025 * t10609;
    let t29922 = t683 * t8605 * t10583;
    let t29934 = t10843 * t214;
    let t29960 = t683 * t2025 * t10579;
    let t29964 = -t8548 * t8536 * t29909 / 2.0 - t29914 / 96.0 - t29917 / 48.0 + 41.0 / 144.0 * t25633 - 5.0 / 432.0 * t25636 - 7.0 / 48.0 * t29922 + t25657 / 72.0 - t25660 / 96.0 - t683 * t686 * t25696 * t1232 / 32.0 + t683 * t3167 * t8580 * t3 / 8.0 - t683 * t686 * t29934 * t674 / 32.0 - t2035 * t2038 * t6701 * t3979 / 48.0 - t683 * t686 * t6701 * t3988 / 64.0 - t683 * t686 * t2044 * t10621 / 32.0 - t683 * t686 * t25666 * t1232 / 32.0 + t683 * t3167 * t8562 * t3 / 8.0 - t29960 / 48.0 - t25672 / 48.0 - t25675 / 96.0;
    (t29908, t29909, t29964)
}
