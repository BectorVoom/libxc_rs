//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1871;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta583(t252: f64, t87230: f64, t13230: f64, t87052: f64, t23168: f64, t25321: f64, t25284: f64, t6579: f64, t13388: f64, t1888: f64, t6646: f64, t13385: f64, t22996: f64, t23185: f64, t4283: f64, t81914: f64, t25300: f64, t81591: f64, t1484: f64, t6552: f64, t6637: f64, t81658: f64, t25303: f64, t13456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87531, t87533, t87535, t87538, t87541) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1871(t252, t87230, t13230, t87052, t23168, t25321, t25284, t6579, t13388, t1888, t6646, t13385, t22996);
        let (t87544, t87546, t87554, t87565, t87575) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1872(t23185, t4283, t81914, t25300, t81591, t1484, t6552, t6637, t81658, t25303, t6579, t13456, t1888, t6646);
    (t87531, t87533, t87535, t87538, t87541, t87544, t87546, t87554, t87565, t87575)
}
