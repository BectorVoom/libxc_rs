//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta943 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3178;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta943(t12226: f64, t1719: f64, t12470: f64, t1744: f64, t12555: f64, t5180: f64, t12486: f64, t300: f64, t12553: f64, t3521: f64, t1261: f64, t1715: f64, t247: f64, t44701: f64, t12809: f64, t12916: f64, t17380: f64, t3568: f64, t3603: f64, t1247: f64, t1796: f64, t42994: f64, t17231: f64, t3172: f64, t1250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58473, t58592, t58647, t58665, t58672, t58708, t58777) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3178(t12226, t1719, t12470, t1744, t12555, t5180, t12486, t300, t12553, t3521, t1261, t1715, t247, t44701);
        let (t58791, t58803, t58824, t58827, t58831) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3179(t12809, t12916, t17380, t3568, t3603, t1247, t1796, t42994, t1261, t17231, t3172, t1250);
    (t58473, t58592, t58647, t58665, t58672, t58708, t58777, t58791, t58803, t58824, t58827, t58831)
}
