//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1618;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1619;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta464(t25338: f64, t6552: f64, t4119: f64, t6554: f64, t6553: f64, t23204: f64, t7479: f64, t23164: f64, t1530: f64, t776: f64, t22960: f64, t10143: f64, t25: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25339, t25341, t25342, t25343, t25345, t25346, t25365) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1618(t25338, t6552, t4119, t6554, t6553, t23204, t7479, t23164, t1530, t776);
        let (t25366, t25373) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1619(t22960, t25365, t10143, t25);
        let t25374 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1620(t1530, t868);
    (t25339, t25341, t25342, t25343, t25345, t25346, t25365, t25366, t25373, t25374)
}
