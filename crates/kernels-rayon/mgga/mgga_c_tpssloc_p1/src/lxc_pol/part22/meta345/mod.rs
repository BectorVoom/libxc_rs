//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1547;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1548;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta345(t213: f64, t5527: f64, t221: f64, t776: f64, t4119: f64, t4128: f64, t12986: f64, t13002: f64, t13005: f64, t13010: f64, t16769: f64, t4127: f64, t9526: f64, t9540: f64, t9542: f64, t9547: f64, t9572: f64, t118: f64, t794: f64, t9549: f64, t16662: f64, t210: f64, t214: f64, t5544: f64, t2576: f64, t2563: f64, t5555: f64, t13014: f64, t13020: f64, t13022: f64, t13027: f64, t787: f64, t9579: f64, t9583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16771, t16773, t16777, t16781) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1547(t213, t5527, t221, t776, t4119, t4128, t12986, t13002, t13005, t13010, t16769, t4127, t9526, t9540, t9542, t9547, t9572);
        let (t16783, t16784, t16787, t16791, t16792, t16794, t16796) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1548(t118, t5527, t794, t9549, t16662, t210, t214, t5544, t2576, t2563, t5555, t213);
        let (t16798, t16803) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1549(t16796, t221, t776, t13014, t13020, t13022, t13027, t16784, t16787, t16792, t16794, t4127, t787, t9579, t9583);
    (t16771, t16773, t16777, t16781, t16783, t16784, t16787, t16791, t16792, t16794, t16798, t16803)
}
