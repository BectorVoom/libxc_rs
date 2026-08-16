//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1127;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta260(t1433: f64, t71: f64, t1458: f64, t89: f64, t1453: f64, t6530: f64, t1484: f64, t25: f64, t6554: f64, t6553: f64, t6552: f64, t1519: f64, t225: f64, t258: f64, t214: f64, t1880: f64, t1527: f64, t6571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7445, t7458, t7464, t7475, t7479) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1127(t1433, t71, t1458, t89, t1453, t6530, t1484, t25, t6554);
        let (t7480, t7481, t7484, t7485, t7486, t7488) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1128(t6553, t7479, t6552, t1519, t225, t258, t214, t1880, t1527, t6571);
    (t7445, t7458, t7464, t7475, t7479, t7480, t7481, t7484, t7485, t7486, t7488)
}
