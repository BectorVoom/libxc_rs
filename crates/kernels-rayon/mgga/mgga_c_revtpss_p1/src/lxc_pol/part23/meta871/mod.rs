//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta871 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2770;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta871(t6880: f64, t9779: f64, t22062: f64, t9775: f64, t13845: f64, t22145: f64, t48100: f64, t22068: f64, t9765: f64, t22052: f64, t3989: f64, t22022: f64, t22061: f64, t808: f64, t9845: f64, t22085: f64, t9962: f64, t22182: f64, t47215: f64, t22021: f64, t9793: f64, t9794: f64, t6876: f64, t9909: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74279, t74281, t74288, t74290, t74292, t74299) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2770(t6880, t9779, t22062, t9775, t13845, t22145, t48100, t22068, t9765, t22052, t3989, t22022);
        let (t74304, t74319, t74322, t74341, t74358) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2771(t22061, t808, t9845, t22085, t9962, t22182, t47215, t22021, t9793, t9794, t6876, t9909);
    (t74279, t74281, t74288, t74290, t74292, t74299, t74304, t74319, t74322, t74341, t74358)
}
