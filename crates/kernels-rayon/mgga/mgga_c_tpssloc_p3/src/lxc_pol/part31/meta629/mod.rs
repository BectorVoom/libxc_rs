//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1887;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta629(t1339: f64, t26288: f64, t550: f64, t57172: f64, t22827: f64, t74366: f64, t1307: f64, t6415: f64, t6420: f64, t1825: f64, t5286: f64, t6936: f64, t57091: f64, t19890: f64, t26309: f64, t236: f64, t6387: f64, t22705: f64, t22852: f64, t19805: f64, t2002: f64, t559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97287, t97291, t97295, t97299, t97303) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1887(t1339, t26288, t550, t57172, t22827, t74366, t1307, t6415, t6420, t1825, t5286, t6936);
        let (t97307, t97310, t97312, t97315, t97318) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1888(t1339, t550, t57091, t6936, t19890, t26309, t236, t6387, t22705, t22852, t19805, t2002, t559);
    (t97287, t97291, t97295, t97299, t97303, t97307, t97310, t97312, t97315, t97318)
}
