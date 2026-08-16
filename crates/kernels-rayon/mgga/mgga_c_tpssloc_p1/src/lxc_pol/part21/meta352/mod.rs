//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1755;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta352(t13123: f64, t2375: f64, t184: f64, t3966: f64, t607: f64, t4194: f64, t12606: f64, t185: f64, t707: f64, t4094: f64, t706: f64, t708: f64, t9924: f64, t9933: f64, t13112: f64, t13114: f64, t13117: f64, t13118: f64, t13121: f64, t13122: f64, t9853: f64, t9859: f64, t9907: f64, t9921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13124, t13125, t13127, t13129, t13130, t13132, t13133) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1755(t13123, t2375, t184, t3966, t607, t4194, t12606, t185, t707, t4094, t706);
        let (t13135, t13136, t13137, t13138) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1756(t13133, t708, t9924, t9933, t13112, t13114, t13117, t13118, t13121, t13122, t13125, t13129, t13132, t9853, t9859, t9907, t9921);
    (t13124, t13125, t13127, t13129, t13130, t13132, t13133, t13135, t13136, t13137, t13138)
}
