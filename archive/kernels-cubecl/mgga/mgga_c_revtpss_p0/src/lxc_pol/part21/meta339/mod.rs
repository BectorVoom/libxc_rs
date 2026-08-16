//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1656;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1657;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1658;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta339<F: Float>(t11410: F, t2970: F, t11132: F, t11337: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11339: F, t11343: F, t11346: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11368: F, t11370: F, t11373: F, t11376: F, t954: F, t2966: F, t944: F, t302: F, t2969: F, t310: F, t2979: F, t964: F, t3011: F, t960: F, t3010: F, t320: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11411, t11422, t11423, t11428) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1656::<F>(t11410, t2970, t11132, t11337, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11339, t11343, t11346);
        let t11443 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1657::<F>(t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368, t11370, t11373, t11376);
        let (t11444, t11445, t11449, t11450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1658::<F>(t11428, t11443, t954, t2966, t944, t302);
        let (t11452, t11453, t11456, t11461, t11465) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1659::<F>(t2969, t310, t11410, t2979, t964, t3011, t960, t3010, t320);
    (t11411, t11422, t11423, t11444, t11445, t11449, t11450, t11452, t11453, t11456, t11461, t11465)
}
