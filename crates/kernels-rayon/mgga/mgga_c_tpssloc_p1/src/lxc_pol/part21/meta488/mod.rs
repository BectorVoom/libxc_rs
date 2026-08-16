//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2090;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta488(t16758: f64, t829: f64, t4234: f64, t4282: f64, t5550: f64, t9573: f64, t213: f64, t5527: f64, t221: f64, t776: f64, t4119: f64, t4128: f64, t12986: f64, t13002: f64, t13005: f64, t13010: f64, t4127: f64, t9526: f64, t9540: f64, t9542: f64, t9547: f64, t9572: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t16759, t16762, t16769, t16771, t16773, t16777) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2090(t16758, t829, t4234, t4282, t5550, t9573, t213, t5527, t221, t776, t4119, t4128);
        let t16781 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2091(t12986, t13002, t13005, t13010, t16769, t16773, t16777, t4127, t9526, t9540, t9542, t9547, t9572);
    (t16759, t16762, t16771, t16773, t16777, t16781)
}
