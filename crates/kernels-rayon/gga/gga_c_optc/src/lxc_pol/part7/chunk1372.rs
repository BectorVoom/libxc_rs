//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1372/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1372(t1179: f64, t27184: f64, t27181: f64, t27134: f64, t1162: f64, t2367: f64, t8533: f64, t11885: f64, t3244: f64, t9069: f64, t11781: f64, t3105: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27299 = t1179 * t27184;
    let t27307 = t1179 * t27181;
    let t27309 = t1179 * t27134;
    let t27318 = t1162 * t2367 * t8533;
    let t27328 = t3244 * t11885 * t9069;
    let t27333 = t11781 * t3105;
    (t27299, t27307, t27309, t27318, t27328, t27333)
}
