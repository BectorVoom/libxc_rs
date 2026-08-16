//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1131;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta263(t6646: f64, t7524: f64, t1888: f64, t1519: f64, t1894: f64, t214: f64, t1880: f64, t1530: f64, t25: f64, t1484: f64, t28: f64, t1458: f64, t88: f64, t1778: f64, t191: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7525, t7526, t7528, t7529, t7530, t7545, t7649, t7656, t7676) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1131(t6646, t7524, t1888, t1519, t1894, t214, t1880, t1530, t25, t1484, t28, t1458, t88);
        let (t7684, t7685) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1132(t1778, t191, t192);
    (t7525, t7526, t7528, t7529, t7530, t7545, t7649, t7656, t7676, t7684, t7685)
}
