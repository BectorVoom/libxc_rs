//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1435;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta463(t12485: f64, t1749: f64, t12428: f64, t1737: f64, t12247: f64, t1719: f64, t12226: f64, t1261: f64, t1715: f64, t247: f64, t44701: f64, t1247: f64, t1796: f64, t42994: f64, t127: f64, t5277: f64, t12851: f64, t1778: f64, t3766: f64, t5219: f64, t5330: f64, t1284: f64, t17306: f64, t3624: f64, t12898: f64, t1804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58262, t58304, t58342, t58473, t58777, t58824) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1435(t12485, t1749, t12428, t1737, t12247, t1719, t12226, t1261, t1715, t247, t44701, t1247, t1796, t42994);
        let (t58895, t59144, t59162, t59411, t59419) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1436(t127, t5277, t12851, t1778, t3766, t5219, t5330, t1284, t17306, t3624, t12898, t1804);
    (t58262, t58304, t58342, t58473, t58777, t58824, t58895, t59144, t59162, t59411, t59419)
}
