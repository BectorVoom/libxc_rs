//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1199/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1199(t18019: f64, t3234: f64, t9189: f64, t1178: f64, t54599: f64, t18030: f64, t12068: f64, t18012: f64, t1107: f64, t17697: f64, t9122: f64, t9124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55210 = t3234 * t9189 * t18019;
    let t55214 = t1178 * t54599;
    let t55262 = t3234 * t9189 * t18030;
    let t55265 = t3234 * t12068 * t18012;
    let t55330 = t1107 * t17697;
    let t55332 = t9122 * t55330 * t9124;
    (t55210, t55214, t55262, t55265, t55330, t55332)
}
