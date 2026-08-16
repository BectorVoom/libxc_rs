//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1806;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta572(t1887: f64, t81959: f64, t22690: f64, t23171: f64, t25319: f64, t23143: f64, t7525: f64, t25238: f64, t6579: f64, t22893: f64, t23164: f64, t25312: f64, t82011: f64, t82039: f64, t25273: f64, t244: f64, t268: f64, t6559: f64, t25250: f64, t87202: f64, t25316: f64, t82038: f64, t23110: f64, t23185: f64, t25272: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87642, t87653, t87666, t87668, t87679) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1806(t1887, t81959, t22690, t23171, t25319, t23143, t7525, t25238, t6579, t22893, t23164, t25312);
        let (t87687, t87708, t87709, t87712, t87714, t87718, t87729) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1807(t82011, t82039, t25273, t6579, t244, t268, t6559, t25250, t87202, t25316, t82038, t23110, t23185, t25272);
    (t87642, t87653, t87666, t87668, t87679, t87687, t87708, t87709, t87712, t87714, t87718, t87729)
}
