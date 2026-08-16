//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta51 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk326;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta51(t500: f64, t111: f64, t88: f64, t522: f64, t588: f64, t592: f64, t521: f64, t750: f64, t17: f64, t67: f64, t758: f64, t172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1256, t1268, t1274, t1276, t1287) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk326(t500, t111, t88, t522, t588, t592, t521, t750);
        let (t1288, t1291, t1293, t1294) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk327(t1287, t17, t521, t67, t758, t172);
    (t1256, t1268, t1274, t1276, t1287, t1288, t1291, t1293, t1294)
}
