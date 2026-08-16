//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1885;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta628(t22765: f64, t6422: f64, t19921: f64, t6952: f64, t19926: f64, t22756: f64, t22783: f64, t6431: f64, t1831: f64, t91160: f64, t19815: f64, t6951: f64, t1369: f64, t1339: f64, t1824: f64, t22827: f64, t5187: f64, t550: f64, t74677: f64, t1307: f64, t3788: f64, t6388: f64, t6427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97253, t97255, t97257, t97259, t97261, t97263, t97265) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1885(t22765, t6422, t19921, t6952, t19926, t22756, t22783, t6431, t1831, t91160, t19815, t6951);
        let (t97266, t97273, t97277, t97281, t97283) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1886(t1369, t97265, t1339, t1824, t22827, t5187, t550, t74677, t1307, t3788, t6388, t22783, t6427);
    (t97253, t97255, t97257, t97259, t97261, t97263, t97266, t97273, t97277, t97281, t97283)
}
