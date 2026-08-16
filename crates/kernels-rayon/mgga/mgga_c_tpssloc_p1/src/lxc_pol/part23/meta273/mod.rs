//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk954;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk955;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta273(t20594: f64, t225: f64, t554: f64, t12215: f64, t1341: f64, t1363: f64, t16285: f64, t1827: f64, t19855: f64, t19940: f64, t19942: f64, t20512: f64, t20516: f64, t20556: f64, t20565: f64, t20570: f64, t3733: f64, t5235: f64, t559: f64, t6390: f64, t6422: f64, t16211: f64, t1831: f64, t19834: f64, t19839: f64, t19841: f64, t19851: f64, t19904: f64, t20433: f64, t20442: f64, t20484: f64, t20508: f64, t3803: f64, t5240: f64, t6427: f64, t6431: f64, t539: f64, t1842: f64, t6439: f64, t12021: f64, t6460: f64, t3887: f64, t553: f64, t12249: f64, t20490: f64, t20495: f64, t3897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20595, t20596, t20599) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk954(t20594, t225, t554, t12215, t1341, t1363, t16285, t1827, t19855, t19940, t19942, t20512, t20516, t20556, t20565, t20570, t3733, t5235, t559, t6390, t6422);
        let t20601 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk955(t1363, t16211, t1831, t19834, t19839, t19841, t19851, t19904, t20433, t20442, t20484, t20508, t20599, t3803, t5240, t6427, t6431);
        let (t20602, t20609, t20613, t20616, t20622, t20625) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk956(t20601, t539, t1842, t6439, t12021, t6460, t3887, t553, t12249, t20490, t20495, t3897);
    (t20595, t20596, t20601, t20602, t20609, t20613, t20616, t20622, t20625)
}
