//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2233;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2234;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2235;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta552(t17331: f64, t225: f64, t480: f64, t1256: f64, t5258: f64, t5262: f64, t1804: f64, t3655: f64, t1786: f64, t1260: f64, t12987: f64, t1774: f64, t3568: f64, t247: f64, t3719: f64, t15687: f64, t3623: f64, t3782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17332, t17333, t17337, t17339, t17340, t17342, t17344) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2233(t17331, t225, t480, t1256, t5258, t5262, t1804, t3655, t1786, t1260, t12987);
        let t17345 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2234(t1774, t3568);
        let (t17347, t17350) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2235(t17345, t247, t3719, t15687, t3623);
        let t17351 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2236(t17350, t3782);
    (t17332, t17333, t17337, t17339, t17340, t17342, t17344, t17345, t17347, t17350, t17351)
}
