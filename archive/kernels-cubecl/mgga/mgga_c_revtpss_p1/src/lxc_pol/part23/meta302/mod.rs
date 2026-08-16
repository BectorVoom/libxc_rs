//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1560;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1561;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta302<F: Float>(t12166: F, t342: F, t11631: F, t12051: F, t1129: F, t3431: F, t408: F, t3434: F, t421: F, t1126: F, t3432: F, t418: F, t240: F, t3698: F, t3361: F, t635: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12167, t12168, t12226, t12227) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1560::<F>(t12166, t342, t11631, t12051, t1129, t3431, t408);
        let (t12230, t12243, t12247, t12248) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1561::<F>(t3434, t421, t1126, t3432, t3431, t418, t408);
        let (t12254, t12256) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1562::<F>(t240, t3698, t3361, t635);
    (t12167, t12168, t12226, t12227, t12230, t12243, t12247, t12248, t12254, t12256)
}
