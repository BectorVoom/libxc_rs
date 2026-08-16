//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1733;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1734;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1735;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1736;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta365(t1065: f64, t3075: f64, t906: f64, t1042: f64, t1047: f64, t1063: f64, t1068: f64, t11977: f64, t11980: f64, t11983: f64, t11989: f64, t11991: f64, t11994: f64, t11999: f64, t12004: f64, t12007: f64, t12010: f64, t12013: f64, t12017: f64, t12021: f64, t3115: f64, t3127: f64, t3130: f64, t3157: f64, t3164: f64, t11642: f64, t11701: f64, t11751: f64, t11799: f64, t11850: f64, t11919: f64, t11976: f64, t225: f64, t385: f64, t3270: f64, t999: f64, t3269: f64, t11804: f64, t996: f64, t1035: f64, t11239: f64, t342: f64, t11247: f64, t378: f64, t3145: f64, t334: f64, t11249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12025, t12026, t12029) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1732(t1065, t3075, t906, t1042, t1047, t1063, t1068, t11977, t11980, t11983, t11989, t11991, t11994, t11999, t12004, t12007, t12010, t12013, t12017, t12021, t3115, t3127, t3130, t3157, t3164);
        let t12032 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1733(t11642, t11701, t11751, t11799, t11850, t11919, t11976, t12029);
        let (t12034, t12039, t12040, t12043, t12046) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1734(t12032, t225, t385, t3270, t999, t3269, t11804, t996, t1035, t11239);
        let t12047 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1735(t12046, t342);
        let (t12048, t12050) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1736(t11247, t378, t3145, t334);
        let t12051 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1737(t11249, t12050);
    (t12025, t12026, t12032, t12034, t12039, t12040, t12043, t12046, t12047, t12048, t12050, t12051)
}
