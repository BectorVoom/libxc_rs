//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2102;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta493(t232: f64, t4119: f64, t2645: f64, t4181: f64, t16891: f64, t2647: f64, t13242: f64, t5591: f64, t13228: f64, t13351: f64, t13222: f64, t16839: f64, t9627: f64, t2632: f64, t4233: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16912, t16914, t16918, t16924, t16928, t16932) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2102(t232, t4119, t2645, t4181, t16891, t2647, t13242, t5591, t13228, t13351, t13222, t16839, t9627);
        let t16935 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2103(t2632, t4233);
    (t16912, t16914, t16918, t16924, t16928, t16932, t16935)
}
