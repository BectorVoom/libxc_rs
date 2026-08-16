//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1288;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta444<F: Float>(t12211: F, t20516: F, t20501: F, t3726: F, t54042: F, t6390: F, t20479: F, t3866: F, t16336: F, t6427: F, t1824: F, t6414: F, t17: F, t20396: F, t750: F, t1358: F, t20596: F, t12283: F, t20442: F, t120: F, t20356: F, t20465: F, t16398: F, t20470: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74393, t74395, t74401, t74403, t74405, t74415) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1288::<F>(t12211, t20516, t20501, t3726, t54042, t6390, t20479, t3866, t16336, t6427, t1824, t6414);
        let (t74496, t74578, t74584, t74592, t74597, t74618) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1289::<F>(t17, t20396, t750, t1358, t20596, t12283, t20442, t120, t20356, t20465, t16398, t20470);
    (t74393, t74395, t74401, t74403, t74405, t74415, t74496, t74578, t74584, t74592, t74597, t74618)
}
