//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1873;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1874;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta408<F: Float>(t1261: F, t13089: F, t3636: F, t3647: F, t12646: F, t247: F, t3719: F, t3367: F, t414: F, t66: F, t12257: F, t1222: F, t1247: F, t1252: F, t13008: F, t13012: F, t13015: F, t13018: F, t13022: F, t13029: F, t13033: F, t13042: F, t13048: F, t13052: F, t13055: F, t13058: F, t13062: F, t13065: F, t13069: F, t13076: F, t13081: F, t13086: F, t3591: F, t3606: F, t3613: F, t3708: F, t5384: F, t12845: F, t12929: F, t13005: F) -> (F, F, F, F, F, F, F) {
        let (t13090, t13092, t13095, t13099) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1873::<F>(t1261, t13089, t3636, t3647, t12646, t247, t3719, t3367, t414);
        let (t13100, t13102, t13105) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1874::<F>(t13099, t66, t12257, t247, t1222, t1247, t1252, t1261, t13008, t13012, t13015, t13018, t13022, t13029, t13033, t13042, t13048, t13052, t13055, t13058, t13062, t13065, t13069, t13076, t13081, t13086, t13090, t13092, t13095, t3591, t3606, t3613, t3708, t5384);
        let t13107 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1875::<F>(t12845, t12929, t13005, t13105);
    (t13090, t13092, t13095, t13099, t13100, t13102, t13107)
}
