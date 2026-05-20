//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1733;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1734;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1735;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1736;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta365<F: Float>(t1065: F, t3075: F, t906: F, t1042: F, t1047: F, t1063: F, t1068: F, t11977: F, t11980: F, t11983: F, t11989: F, t11991: F, t11994: F, t11999: F, t12004: F, t12007: F, t12010: F, t12013: F, t12017: F, t12021: F, t3115: F, t3127: F, t3130: F, t3157: F, t3164: F, t11642: F, t11701: F, t11751: F, t11799: F, t11850: F, t11919: F, t11976: F, t225: F, t385: F, t3270: F, t999: F, t3269: F, t11804: F, t996: F, t1035: F, t11239: F, t342: F, t11247: F, t378: F, t3145: F, t334: F, t11249: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12025, t12026, t12029) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1732::<F>(t1065, t3075, t906, t1042, t1047, t1063, t1068, t11977, t11980, t11983, t11989, t11991, t11994, t11999, t12004, t12007, t12010, t12013, t12017, t12021, t3115, t3127, t3130, t3157, t3164);
        let t12032 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1733::<F>(t11642, t11701, t11751, t11799, t11850, t11919, t11976, t12029);
        let (t12034, t12039, t12040, t12043, t12046) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1734::<F>(t12032, t225, t385, t3270, t999, t3269, t11804, t996, t1035, t11239);
        let t12047 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1735::<F>(t12046, t342);
        let (t12048, t12050) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1736::<F>(t11247, t378, t3145, t334);
        let t12051 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1737::<F>(t11249, t12050);
    (t12025, t12026, t12032, t12034, t12039, t12040, t12043, t12046, t12047, t12048, t12050, t12051)
}
