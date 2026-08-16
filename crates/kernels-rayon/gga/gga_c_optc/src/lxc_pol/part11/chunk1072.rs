//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1072/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1072(t1464: f64, t8785: f64, t1476: f64, t8749: f64, t8847: f64, t8697: f64, t1506: f64, t26869: f64, t8: f64, t8425: f64, t1121: f64, t1508: f64, t8528: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34801 = t1464 * t8785;
    let t34813 = t1476 * t8749;
    let t34816 = t1464 * t8847;
    let t34829 = t1476 * t8697;
    let t35165 = t26869 * t1506;
    let t35363 = t8 * t8425;
    let t35379 = t1121 * t8528 * t1508;
    (t34801, t34813, t34816, t34829, t35165, t35363, t35379)
}
