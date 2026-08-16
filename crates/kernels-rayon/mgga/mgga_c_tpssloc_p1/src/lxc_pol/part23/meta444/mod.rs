//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1288;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta444(t12211: f64, t20516: f64, t20501: f64, t3726: f64, t54042: f64, t6390: f64, t20479: f64, t3866: f64, t16336: f64, t6427: f64, t1824: f64, t6414: f64, t17: f64, t20396: f64, t750: f64, t1358: f64, t20596: f64, t12283: f64, t20442: f64, t120: f64, t20356: f64, t20465: f64, t16398: f64, t20470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74393, t74395, t74401, t74403, t74405, t74415) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1288(t12211, t20516, t20501, t3726, t54042, t6390, t20479, t3866, t16336, t6427, t1824, t6414);
        let (t74496, t74578, t74584, t74592, t74597, t74618) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1289(t17, t20396, t750, t1358, t20596, t12283, t20442, t120, t20356, t20465, t16398, t20470);
    (t74393, t74395, t74401, t74403, t74405, t74415, t74496, t74578, t74584, t74592, t74597, t74618)
}
