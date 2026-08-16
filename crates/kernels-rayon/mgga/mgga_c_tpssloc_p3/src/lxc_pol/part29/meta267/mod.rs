//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1256;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1257;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta267(t510: f64, t7467: f64, t652: f64, t1484: f64, t25: f64, t1915: f64, t6554: f64, t6553: f64, t6552: f64, t1519: f64, t225: f64, t258: f64, t214: f64, t1880: f64, t1527: f64, t6571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7468 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1256(t510, t7467);
        let (t7470, t7475, t7476, t7479) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1257(t652, t7468, t1484, t25, t1915, t6554);
        let (t7480, t7481, t7484, t7485, t7486, t7488) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1258(t6553, t7479, t6552, t1519, t225, t258, t214, t1880, t1527, t6571);
    (t7468, t7470, t7475, t7476, t7479, t7480, t7481, t7484, t7485, t7486, t7488)
}
