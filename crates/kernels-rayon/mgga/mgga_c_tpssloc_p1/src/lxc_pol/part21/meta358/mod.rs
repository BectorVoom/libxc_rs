//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1774;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta358(t120: f64, t4119: f64, t2645: f64, t829: f64, t2679: f64, t4248: f64, t13242: f64, t4180: f64, t4181: f64, t4240: f64, t9638: f64, t2647: f64, t10007: f64, t4191: f64, t13275: f64, t13277: f64, t13280: f64, t13283: f64, t13287: f64, t13289: f64, t13293: f64, t13297: f64, t1512: f64, t2571: f64, t2618: f64, t2635: f64, t2643: f64, t2686: f64, t4167: f64, t4236: f64, t4250: f64, t9559: f64, t9613: f64, t9642: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13302, t13306, t13312, t13316, t13320, t13322) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1774(t120, t4119, t2645, t829, t2679, t4248, t13242, t4180, t4181, t4240, t9638, t2647);
        let (t13326, t13330, t13331) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1775(t10007, t2645, t4181, t4191, t9638, t13275, t13277, t13280, t13283, t13287, t13289, t13293, t13297, t13302, t13306, t13312, t13316, t13320, t13322, t1512, t2571, t2618, t2635, t2643, t2686, t4167, t4236, t4250, t9559, t9613, t9642);
    (t13302, t13306, t13312, t13316, t13320, t13322, t13326, t13330, t13331)
}
