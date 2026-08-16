//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1600;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1601;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1602;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1603;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta323(t11044: f64, t2467: f64, t2828: f64, t676: f64, t123: f64, t2465: f64, t11004: f64, t11010: f64, t11013: f64, t11017: f64, t11019: f64, t11022: f64, t11026: f64, t11030: f64, t11032: f64, t11037: f64, t11040: f64, t213: f64, t257: f64, t2765: f64, t2772: f64, t2829: f64, t865: f64, t11002: f64, t2408: f64, t890: f64, t2410: f64, t261: f64, t2411: f64, t2832: f64, t892: f64, t10552: f64, t10554: f64, t10557: f64, t10560: f64, t10562: f64, t10564: f64, t10627: f64, t1940: f64, t198: f64, t207: f64, t2394: f64, t2403: f64, t2404: f64, t2430: f64, t262: f64, t4541: f64, t775: f64, t9394: f64, t10566: f64, t10568: f64, t10570: f64, t10575: f64, t10577: f64, t10580: f64, t10582: f64, t10584: f64, t9514: f64, t9517: f64, t9521: f64, t10586: f64, t10589: f64, t10592: f64, t10594: f64, t10596: f64, t10598: f64, t10602: f64, t10604: f64, t10607: f64, t10609: f64, t10611: f64, t10614: f64, t9524: f64, t9542: f64, t10493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11045, t11049, t11050, t11051, t11053) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1600(t11044, t2467, t2828, t676, t123, t2465, t11004, t11010, t11013, t11017, t11019, t11022, t11026, t11030, t11032, t11037, t11040, t213, t257, t2765, t2772, t2829, t865);
        let (t11054, t11061, t11064, t11075, t11082) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1601(t11002, t11053, t2408, t890, t2410, t261, t2411, t2832, t892, t10552, t10554, t10557, t10560, t10562, t10564, t10627, t1940, t198, t207, t2394, t2403, t2404, t2430, t262, t4541, t775, t9394);
        let (t11084, t11092) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1602(t2408, t2411, t262, t775, t10566, t10568, t10570, t10575, t10577, t10580, t10582, t10584, t2403, t2430, t4541, t9514, t9517, t9521);
        let t11093 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1603(t10586, t10589, t10592, t10594, t10596, t10598, t10602, t10604, t10607, t10609, t10611, t10614, t9524, t9542);
        let t11095 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1604(t10493, t11082, t11092, t11093);
    (t11045, t11049, t11050, t11051, t11054, t11061, t11064, t11075, t11084, t11095)
}
