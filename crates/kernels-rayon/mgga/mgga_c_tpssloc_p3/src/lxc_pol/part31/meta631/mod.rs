//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1891;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta631(t19996: f64, t6952: f64, t26257: f64, t5310: f64, t1358: f64, t28088: f64, t22852: f64, t3792: f64, t80798: f64, t97312: f64, t22705: f64, t236: f64, t550: f64, t6414: f64, t22765: f64, t6417: f64, t6390: f64, t80997: f64, t81000: f64, t1351: f64, t3788: f64, t6388: f64, t6936: f64, t19958: f64, t22833: f64, t22797: f64, t6375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97359, t97361, t97363, t97367, t97372) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1891(t19996, t6952, t26257, t5310, t1358, t28088, t22852, t3792, t80798, t97312, t22705, t236, t550, t6414);
        let (t97378, t97380, t97382, t97387, t97389, t97394) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1892(t22765, t6417, t6390, t80997, t81000, t1351, t3788, t6388, t6936, t19958, t22833, t22797, t6375);
    (t97359, t97361, t97363, t97367, t97372, t97378, t97380, t97382, t97387, t97389, t97394)
}
