//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1853;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1854;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1855;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta398<F: Float>(t1250: F, t12718: F, t3720: F, t126: F, t482: F, t828: F, t3722: F, t3718: F, t1214: F, t2251: F, t5268: F, t1042: F, t11231: F, t1261: F, t12847: F, t12853: F, t12855: F, t12858: F, t12862: F, t12866: F, t12868: F, t12872: F, t12876: F, t12882: F, t12887: F, t12890: F, t12893: F, t12895: F, t12900: F, t12902: F, t12905: F, t12907: F, t12910: F, t3711: F, t484: F, t5331: F, t5340: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12911, t12912, t12915) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1853::<F>(t1250, t12718, t3720, t126, t482);
        let t12916 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1854::<F>(t12915, t828);
        let (t12917, t12918, t12920) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1855::<F>(t12916, t3722, t3718, t1214, t2251);
        let (t12921, t12922, t12925, t12926, t12929) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1856::<F>(t12920, t5268, t1042, t11231, t1261, t12847, t12853, t12855, t12858, t12862, t12866, t12868, t12872, t12876, t12882, t12887, t12890, t12893, t12895, t12900, t12902, t12905, t12907, t12910, t12912, t12918, t3711, t3718, t484, t5331, t5340);
    (t12911, t12912, t12915, t12916, t12917, t12918, t12920, t12921, t12922, t12925, t12926, t12929)
}
