//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta740 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2604;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta740(t11745: f64, t15737: f64, t1227: f64, t13969: f64, t15649: f64, t43763: f64, t44827: f64, t11539: f64, t1174: f64, t14740: f64, t14731: f64, t135: f64, t15666: f64, t11665: f64, t15572: f64, t3515: f64, t4983: f64, t49850: f64, t11818: f64, t1213: f64, t248: f64, t5012: f64, t11801: f64, t5024: f64, t11820: f64, t5019: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52908, t52917, t52919, t52926, t52932, t52935) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2604(t11745, t15737, t1227, t13969, t15649, t43763, t44827, t11539, t1174, t14740, t14731, t135, t15666);
        let (t52942, t52952, t52973, t52975, t52987) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2605(t11665, t15572, t3515, t4983, t49850, t11818, t1213, t248, t5012, t11801, t5024, t11820, t5019);
    (t52908, t52917, t52919, t52926, t52932, t52935, t52942, t52952, t52973, t52975, t52987)
}
