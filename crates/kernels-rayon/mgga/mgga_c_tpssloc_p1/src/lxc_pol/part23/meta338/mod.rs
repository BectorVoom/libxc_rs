//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1111;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1112;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1113;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta338(t1294: f64, t39344: f64, t9810: f64, t9844: f64, t39321: f64, t677: f64, t9713: f64, t3684: f64, t181: f64, t2558: f64, t686: f64, t1291: f64, t2369: f64, t9720: f64, t9843: f64, t2411: f64, t2414: f64, t39246: f64, t2508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39346, t39347, t39349, t39354, t39356, t39358, t39360) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1111(t1294, t39344, t9810, t9844, t39321, t677, t9713, t3684, t181, t2558, t686, t1291);
        let (t39362, t39364, t39373) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1112(t2369, t9720, t9843, t1294, t2411, t2414, t39246);
        let (t39377, t39378) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1113(t2508, t2369);
    (t39346, t39347, t39349, t39354, t39356, t39358, t39360, t39362, t39364, t39373, t39377, t39378)
}
