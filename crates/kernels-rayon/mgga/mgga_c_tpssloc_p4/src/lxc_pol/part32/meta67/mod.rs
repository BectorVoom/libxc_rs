//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta67 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk444;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk445;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk446;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta67(t25: f64, t28: f64, t1268: f64, t650: f64, t671: f64, t522: f64, t588: f64, t592: f64, t514: f64, t606: f64, t1081: f64, t517: f64, t157: f64, zeta_threshold: f64, t184: f64, t17: f64, t521: f64, t750: f64, t182: f64, t67: f64, t758: f64, t172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1271, t1274, t1276, t1284) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk444(t25, t28, t1268, t650, t671, t522, t588, t592, t514, t606, t1081, t517, t157, zeta_threshold);
        let t1285 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk445(t1284, t184);
        let (t1286, t1287) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk446(t1285, t17, t521, t750);
        let (t1288, t1290, t1291, t1293, t1294) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk447(t1287, t17, t1284, t182, t521, t67, t758, t172);
    (t1271, t1274, t1276, t1284, t1285, t1286, t1287, t1288, t1290, t1291, t1293, t1294)
}
