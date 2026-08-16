//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1504;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta262(t9397: f64, t195: f64, t40: f64, t197: f64, t52: f64, t138: f64, t2409: f64, t125: f64, t2412: f64, t701: f64, t2414: f64, t2519: f64, t751: f64, t2393: f64, t763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9398, t9427, t9438, t9452, t9453, t9454, t9455, t9457) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1504(t9397, t195, t40, t197, t52, t138, t2409, t125, t2412, t701, t2414);
        let (t9462, t9467) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1505(t2519, t751, t2393, t763);
    (t9398, t9427, t9438, t9452, t9453, t9454, t9455, t9457, t9462, t9467)
}
