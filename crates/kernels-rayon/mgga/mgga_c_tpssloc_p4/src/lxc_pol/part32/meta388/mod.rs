//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1468;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1469;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta388(t232: f64, t4119: f64, t2645: f64, t4181: f64, t16891: f64, t2647: f64, t13242: f64, t5591: f64, t13228: f64, t13351: f64, t13222: f64, t16839: f64, t9627: f64, t2632: f64, t4233: f64, t4180: f64, t2639: f64, t5619: f64, t5614: f64, t1484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16914, t16918, t16924, t16927, t16928, t16932) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1468(t232, t4119, t2645, t4181, t16891, t2647, t13242, t5591, t13228, t13351, t13222, t16839, t9627);
        let (t16935, t16937, t16940, t16942, t16944) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1469(t2632, t4233, t4180, t4181, t2639, t5619, t5614, t1484, t4119);
    (t16914, t16918, t16924, t16927, t16928, t16932, t16935, t16937, t16940, t16942, t16944)
}
