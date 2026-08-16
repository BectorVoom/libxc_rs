//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1865;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta580(t13184: f64, t221: f64, t87420: f64, t25120: f64, t6604: f64, t81962: f64, t13196: f64, t25119: f64, t841: f64, t13204: f64, t6581: f64, t7500: f64, t81911: f64, t22690: f64, t23122: f64, t4119: f64, t25064: f64, t81902: f64, t23077: f64, t6646: f64, t23098: f64, t7496: f64, t6590: f64, t25130: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87422, t87425, t87428, t87430, t87432) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1865(t13184, t221, t87420, t25120, t6604, t81962, t13196, t25119, t841, t13204, t6581, t7500, t81911);
        let (t87443, t87445, t87449, t87453) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1866(t22690, t23122, t4119, t841, t25064, t81902, t23077, t6646, t23098, t7496, t6590, t25130);
    (t87422, t87425, t87428, t87430, t87432, t87443, t87445, t87449, t87453)
}
