//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 531/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk531(t1199: f64, t1561: f64, t1213: f64, t1574: f64, t490: f64, t1214: f64, t1570: f64, t1218: f64, t491: f64, t1217: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4249 = t1561 * t1199;
    let t4275 = t1574 * t1213;
    let t4276 = t490 * t4275;
    let t4278 = t1570 * t1214;
    let t4280 = t1218 * t491;
    let t4281 = t1217 * t4280;
    (t4249, t4275, t4276, t4278, t4280, t4281)
}
