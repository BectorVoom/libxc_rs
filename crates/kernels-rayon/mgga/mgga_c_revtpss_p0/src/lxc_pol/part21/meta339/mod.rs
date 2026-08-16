//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1656;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1657;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1658;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta339(t11410: f64, t2970: f64, t11132: f64, t11337: f64, t11158: f64, t11162: f64, t11167: f64, t11316: f64, t11319: f64, t11322: f64, t11326: f64, t11329: f64, t11332: f64, t11339: f64, t11343: f64, t11346: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11171: f64, t11356: f64, t11359: f64, t11366: f64, t11368: f64, t11370: f64, t11373: f64, t11376: f64, t954: f64, t2966: f64, t944: f64, t302: f64, t2969: f64, t310: f64, t2979: f64, t964: f64, t3011: f64, t960: f64, t3010: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11411, t11422, t11423, t11428) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1656(t11410, t2970, t11132, t11337, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11339, t11343, t11346);
        let t11443 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1657(t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368, t11370, t11373, t11376);
        let (t11444, t11445, t11449, t11450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1658(t11428, t11443, t954, t2966, t944, t302);
        let (t11452, t11453, t11456, t11461, t11465) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1659(t2969, t310, t11410, t2979, t964, t3011, t960, t3010, t320);
    (t11411, t11422, t11423, t11444, t11445, t11449, t11450, t11452, t11453, t11456, t11461, t11465)
}
