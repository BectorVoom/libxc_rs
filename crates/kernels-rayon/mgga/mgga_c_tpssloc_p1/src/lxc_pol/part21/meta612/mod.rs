//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2380;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2381;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta612(t2411: f64, t2414: f64, t701: f64, t9777: f64, t2405: f64, t2415: f64, t9453: f64, t2225: f64, t3824: f64, t1287: f64, t9214: f64, t12129: f64, t588: f64, t39033: f64, t522: f64, t1285: f64, t9216: f64, t9218: f64, t16: f64, t185: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t39590 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2380(t2411, t2414, t701, t9777);
        let t39593 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2381(t2405, t2415, t9453);
        let (t39595, t39596, t39601, t39603, t39609, t39611, t39615) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2382(t2225, t3824, t1287, t9214, t12129, t588, t39033, t522, t1285, t9216, t9218, t16, t185, t520);
    (t39590, t39593, t39595, t39596, t39601, t39603, t39609, t39611, t39615)
}
