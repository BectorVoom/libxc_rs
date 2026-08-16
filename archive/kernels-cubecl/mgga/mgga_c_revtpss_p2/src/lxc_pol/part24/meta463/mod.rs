//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1435;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta463<F: Float>(t12485: F, t1749: F, t12428: F, t1737: F, t12247: F, t1719: F, t12226: F, t1261: F, t1715: F, t247: F, t44701: F, t1247: F, t1796: F, t42994: F, t127: F, t5277: F, t12851: F, t1778: F, t3766: F, t5219: F, t5330: F, t1284: F, t17306: F, t3624: F, t12898: F, t1804: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t58262, t58304, t58342, t58473, t58777, t58824) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1435::<F>(t12485, t1749, t12428, t1737, t12247, t1719, t12226, t1261, t1715, t247, t44701, t1247, t1796, t42994);
        let (t58895, t59144, t59162, t59411, t59419) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1436::<F>(t127, t5277, t12851, t1778, t3766, t5219, t5330, t1284, t17306, t3624, t12898, t1804);
    (t58262, t58304, t58342, t58473, t58777, t58824, t58895, t59144, t59162, t59411, t59419)
}
