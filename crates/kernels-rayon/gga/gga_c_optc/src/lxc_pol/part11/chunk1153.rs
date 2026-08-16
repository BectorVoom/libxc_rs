//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1153/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1153(t17219: f64, t2721: f64, t8152: f64, t14339: f64, t3884: f64, t42111: f64, t17169: f64, t17125: f64, t16988: f64, t864: f64, t7380: f64, t857: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51355 = t2721 * t8152 * t17219;
    let t51360 = t3884 * t42111 * t14339;
    let t51363 = t2721 * t8152 * t17169;
    let t51368 = t2721 * t8152 * t17125;
    let t51399 = t864 * t16988;
    let t51400 = t51399 * t7380;
    let t51450 = t857 * t16988;
    (t51355, t51360, t51363, t51368, t51399, t51400, t51450)
}
