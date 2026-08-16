//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1557;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1558;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta349(t16891: f64, t4180: f64, t4182: f64, t120: f64, t5527: f64, t829: f64, t9646: f64, t5544: f64, t2645: f64, t16839: f64, t2647: f64, t13177: f64, t13251: f64, t13260: f64, t13275: f64, t13277: f64, t13280: f64, t13287: f64, t13320: f64, t13330: f64, t1512: f64, t16872: f64, t16877: f64, t16879: f64, t16888: f64, t2643: f64, t4167: f64, t4178: f64, t4191: f64, t4236: f64, t4240: f64, t4250: f64, t831: f64, t232: f64, t4119: f64, t4181: f64, t13242: f64, t5591: f64, t13228: f64, t13351: f64, t13222: f64, t9627: f64, t2632: f64, t4233: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16893, t16898, t16903, t16907, t16910) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1557(t16891, t4180, t4182, t120, t5527, t829, t9646, t5544, t2645, t16839, t2647, t13177, t13251, t13260, t13275, t13277, t13280, t13287, t13320, t13330, t1512, t16872, t16877, t16879, t16888, t2643, t4167, t4178, t4191, t4236, t4240, t4250, t831);
        let (t16912, t16914, t16918, t16924, t16928, t16932) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1558(t232, t4119, t2645, t4181, t16891, t2647, t13242, t5591, t13228, t13351, t13222, t16839, t9627);
        let t16935 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1559(t2632, t4233);
    (t16893, t16898, t16903, t16907, t16910, t16912, t16914, t16918, t16924, t16928, t16932, t16935)
}
