//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 776/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk776(t1347: f64, t2001: f64, t1352: f64, t1998: f64, t1446: f64, t1423: f64, t542: f64, t7614: f64, t537: f64, t532: f64, t7605: f64, t1569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8704 = t2001 * t1347;
    let t8706 = t1998 * t1352;
    let t8708 = t2001 * t1446;
    let t8710 = t1998 * t1423;
    let t8712 = t7614 * t542;
    let t8714 = t7614 * t537;
    let t8716 = t7614 * t532;
    let t8718 = t7605 * t532;
    let t8720 = t2001 * t1569;
    (t8704, t8706, t8708, t8710, t8712, t8714, t8716, t8718, t8720)
}
