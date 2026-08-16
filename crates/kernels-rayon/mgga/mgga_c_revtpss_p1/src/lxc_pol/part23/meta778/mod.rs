//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta778 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2582;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta778(t12553: f64, t300: f64, t3521: f64, t1261: f64, t1715: f64, t247: f64, t44701: f64, t1247: f64, t1796: f64, t42994: f64, t3718: f64, t44546: f64, t5347: f64, t17361: f64, t3708: f64, t3625: f64, t44250: f64, t5401: f64, t127: f64, t5277: f64, t17550: f64, t372: f64, t3623: f64, t53667: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58672, t58708, t58777, t58824, t58850) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2582(t12553, t300, t3521, t1261, t1715, t247, t44701, t1247, t1796, t42994, t3718, t44546, t5347);
        let (t58851, t58883, t58889, t58895, t58899, t58919) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2583(t58850, t17361, t3708, t3625, t44250, t5401, t127, t5277, t17550, t372, t3623, t53667);
    (t58672, t58708, t58777, t58824, t58851, t58883, t58889, t58895, t58899, t58919)
}
