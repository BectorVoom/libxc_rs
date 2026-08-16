//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1365/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1365(t2856: f64, t4356: f64, t2855: f64, t3107: f64, t26889: f64, t1111: f64, t11885: f64, t8498: f64, t8493: f64, t9142: f64, t140: f64, t24563: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t27148 = t4356 * t2856;
    let t27152 = t3107 * t2855;
    let t27153 = t27152 * t26889;
    let t27158 = t1111 * t11885 * t8498;
    let t27167 = t1111 * t9142 * t8493;
    let t27173 = t446 * t24563 * t140;
    (t27148, t27153, t27158, t27167, t27173)
}
