//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1887;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta629<F: Float>(t1339: F, t26288: F, t550: F, t57172: F, t22827: F, t74366: F, t1307: F, t6415: F, t6420: F, t1825: F, t5286: F, t6936: F, t57091: F, t19890: F, t26309: F, t236: F, t6387: F, t22705: F, t22852: F, t19805: F, t2002: F, t559: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97287, t97291, t97295, t97299, t97303) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1887::<F>(t1339, t26288, t550, t57172, t22827, t74366, t1307, t6415, t6420, t1825, t5286, t6936);
        let (t97307, t97310, t97312, t97315, t97318) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1888::<F>(t1339, t550, t57091, t6936, t19890, t26309, t236, t6387, t22705, t22852, t19805, t2002, t559);
    (t97287, t97291, t97295, t97299, t97303, t97307, t97310, t97312, t97315, t97318)
}
