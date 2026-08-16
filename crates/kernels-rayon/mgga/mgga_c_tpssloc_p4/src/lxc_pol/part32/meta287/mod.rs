//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1294;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1295;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1296;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1297;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta287(t655: f64, t93: f64, t94: f64, t101: f64, t102: f64, t195: f64, t40: f64, t197: f64, t52: f64, t138: f64, t2409: f64, t125: f64, t2412: f64, t701: f64, t2414: f64, t2393: f64, t763: f64, t2374: f64, t702: f64, t2411: f64, t681: f64, t141: f64, t2413: f64, t2508: f64, t738: f64, t2369: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9365, t9384, t9398, t9427, t9438, t9453) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1294(t655, t93, t94, t101, t102, t195, t40, t197, t52, t138, t2409, t125);
        let (t9454, t9457) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1295(t2412, t701, t2414, t9453);
        let (t9467, t9469, t9476) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1296(t2393, t763, t2374, t702, t9454, t2411);
        let t9484 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1297(t2409, t681, t125, t141, t2413, t9454);
        let (t9489, t9490) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1298(t2508, t738, t2369, t745);
    (t9365, t9384, t9398, t9427, t9438, t9457, t9467, t9469, t9476, t9484, t9489, t9490)
}
