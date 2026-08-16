//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk803;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk804;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk805;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta159(t3493: f64, t475: f64, t1214: f64, t248: f64, t3030: f64, t466: f64, t3032: f64, t1208: f64, t476: f64, t478: f64, t3036: f64, t483: f64, t1215: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3494, t3496, t3499, t3500, t3502, t3503, t3504) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk803(t3493, t475, t1214, t248, t3030, t466, t3032, t1208, t476, t478, t3036, t483);
        let (t3505, t3506) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk804(t3503, t3504, t3500);
        let t3507 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk805(t1215);
        let t3508 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk806(t475);
    (t3494, t3496, t3499, t3500, t3502, t3503, t3504, t3505, t3506, t3507, t3508)
}
