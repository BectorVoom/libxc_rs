//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta778 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2582;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta778<F: Float>(t12553: F, t300: F, t3521: F, t1261: F, t1715: F, t247: F, t44701: F, t1247: F, t1796: F, t42994: F, t3718: F, t44546: F, t5347: F, t17361: F, t3708: F, t3625: F, t44250: F, t5401: F, t127: F, t5277: F, t17550: F, t372: F, t3623: F, t53667: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t58672, t58708, t58777, t58824, t58850) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2582::<F>(t12553, t300, t3521, t1261, t1715, t247, t44701, t1247, t1796, t42994, t3718, t44546, t5347);
        let (t58851, t58883, t58889, t58895, t58899, t58919) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2583::<F>(t58850, t17361, t3708, t3625, t44250, t5401, t127, t5277, t17550, t372, t3623, t53667);
    (t58672, t58708, t58777, t58824, t58851, t58883, t58889, t58895, t58899, t58919)
}
