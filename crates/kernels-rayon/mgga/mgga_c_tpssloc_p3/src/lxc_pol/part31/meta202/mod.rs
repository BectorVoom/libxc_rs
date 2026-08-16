//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk903;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk904;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk905;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk906;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta202(t25: f64, t28: f64, t5397: f64, zeta_threshold: f64, t31: f64, t65: f64, t1410: f64, t1426: f64, t2267: f64, t5392: f64, t43: f64, t48: f64, t480: f64, t2274: f64, t55: f64, sigma2: f64, t1420: f64, t1423: f64, t2282: f64, t39: f64, t51: f64, t56: f64, t33: f64, t2291: f64, t634: f64, t2298: f64, t638: f64, t72: f64, t1411: f64, t1427: f64, t1434: f64, t5393: f64, t66: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5398 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk903(t25, t28, t5397, zeta_threshold);
        let t5399 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk904(t31, t5398);
        let (t5400, t5403, t5408, t5411, t5416, t5421, t5424) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk905(t5399, t65, t1410, t1426, t2267, t5392, t43, t5398, t48, t480, t2274, t55, sigma2);
        let (t5427, t5428) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk906(t1420, t1423, t2282, t39, t51, t5408, t5411, t5416, t5421, t5424, t56, t33);
        let (t5441, t5442, t5445) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk907(t2291, t5392, t5398, t634, t2298, t638, t72, t1411, t1427, t1434, t5393, t5400, t5403, t5428, t66, t80);
    (t5398, t5399, t5400, t5403, t5408, t5411, t5416, t5427, t5428, t5441, t5442, t5445)
}
