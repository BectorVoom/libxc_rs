//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1600;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1601;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1602;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1603;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta323<F: Float>(t11044: F, t2467: F, t2828: F, t676: F, t123: F, t2465: F, t11004: F, t11010: F, t11013: F, t11017: F, t11019: F, t11022: F, t11026: F, t11030: F, t11032: F, t11037: F, t11040: F, t213: F, t257: F, t2765: F, t2772: F, t2829: F, t865: F, t11002: F, t2408: F, t890: F, t2410: F, t261: F, t2411: F, t2832: F, t892: F, t10552: F, t10554: F, t10557: F, t10560: F, t10562: F, t10564: F, t10627: F, t1940: F, t198: F, t207: F, t2394: F, t2403: F, t2404: F, t2430: F, t262: F, t4541: F, t775: F, t9394: F, t10566: F, t10568: F, t10570: F, t10575: F, t10577: F, t10580: F, t10582: F, t10584: F, t9514: F, t9517: F, t9521: F, t10586: F, t10589: F, t10592: F, t10594: F, t10596: F, t10598: F, t10602: F, t10604: F, t10607: F, t10609: F, t10611: F, t10614: F, t9524: F, t9542: F, t10493: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11045, t11049, t11050, t11051, t11053) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1600::<F>(t11044, t2467, t2828, t676, t123, t2465, t11004, t11010, t11013, t11017, t11019, t11022, t11026, t11030, t11032, t11037, t11040, t213, t257, t2765, t2772, t2829, t865);
        let (t11054, t11061, t11064, t11075, t11082) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1601::<F>(t11002, t11053, t2408, t890, t2410, t261, t2411, t2832, t892, t10552, t10554, t10557, t10560, t10562, t10564, t10627, t1940, t198, t207, t2394, t2403, t2404, t2430, t262, t4541, t775, t9394);
        let (t11084, t11092) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1602::<F>(t2408, t2411, t262, t775, t10566, t10568, t10570, t10575, t10577, t10580, t10582, t10584, t2403, t2430, t4541, t9514, t9517, t9521);
        let t11093 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1603::<F>(t10586, t10589, t10592, t10594, t10596, t10598, t10602, t10604, t10607, t10609, t10611, t10614, t9524, t9542);
        let t11095 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1604::<F>(t10493, t11082, t11092, t11093);
    (t11045, t11049, t11050, t11051, t11054, t11061, t11064, t11075, t11084, t11095)
}
