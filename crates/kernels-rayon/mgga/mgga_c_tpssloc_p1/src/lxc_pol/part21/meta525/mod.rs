//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2179;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta525(t248: f64, t3051: f64, t5681: f64, t1041: f64, t1616: f64, t4338: f64, t10408: f64, t1409: f64, t14219: f64, t14218: f64, t3071: f64, t2940: f64, t5804: f64, t14459: f64, t4496: f64, t959: f64, t17194: f64, t17197: f64, t17209: f64, t17301: f64, t17303: f64, t17306: f64, t17372: f64, t17374: f64, t17377: f64, t17379: f64, t17425: f64, t17427: f64, t17561: f64, t17563: f64, t17568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17906, t17907, t17919, t17920, t17923, t17924, t17925, t17929) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2179(t248, t3051, t5681, t1041, t1616, t4338, t10408, t1409, t14219, t14218, t3071, t2940, t5804);
        let (t17930, t17932, t17933) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2180(t14459, t4496, t959, t17194, t17197, t17209, t17301, t17303, t17306, t17372, t17374, t17377, t17379, t17425, t17427, t17561, t17563, t17568, t17929);
    (t17906, t17907, t17919, t17920, t17923, t17924, t17925, t17929, t17930, t17932, t17933)
}
