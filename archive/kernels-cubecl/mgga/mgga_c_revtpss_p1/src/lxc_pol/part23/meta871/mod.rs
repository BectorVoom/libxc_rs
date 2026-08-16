//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta871 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2770;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta871<F: Float>(t6880: F, t9779: F, t22062: F, t9775: F, t13845: F, t22145: F, t48100: F, t22068: F, t9765: F, t22052: F, t3989: F, t22022: F, t22061: F, t808: F, t9845: F, t22085: F, t9962: F, t22182: F, t47215: F, t22021: F, t9793: F, t9794: F, t6876: F, t9909: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t74279, t74281, t74288, t74290, t74292, t74299) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2770::<F>(t6880, t9779, t22062, t9775, t13845, t22145, t48100, t22068, t9765, t22052, t3989, t22022);
        let (t74304, t74319, t74322, t74341, t74358) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2771::<F>(t22061, t808, t9845, t22085, t9962, t22182, t47215, t22021, t9793, t9794, t6876, t9909);
    (t74279, t74281, t74288, t74290, t74292, t74299, t74304, t74319, t74322, t74341, t74358)
}
