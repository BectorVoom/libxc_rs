//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk606;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk607;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk608;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk609;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta99(t107: f64, t2281: f64, t626: f64, t667: f64, t106: f64, t655: f64, t94: f64, t102: f64, t177: f64, t738: f64, t745: f64, t746: f64, t761: f64, t118: f64, t187: f64, t677: f64, t763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2327, t2328, t2331) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk606(t107, t2281, t626, t667, t106, t655);
        let (t2341, t2349, t2368, t2369) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk607(t94, t102, t177, t738, t745);
        let t2371 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk608(t2368, t2369, t746);
        let (t2373, t2374) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk609(t2371, t761, t118, t187);
        let t2375 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk610(t677, t763);
    (t2327, t2328, t2331, t2341, t2349, t2368, t2369, t2371, t2373, t2374, t2375)
}
