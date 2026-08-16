//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 491/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk491(t446: f64, t5672: f64, t6199: f64, t1392: f64, t1515: f64, t1516: f64, t1430: f64, t1475: f64, t221: f64, t209: f64, t589: f64, t1501: f64) -> (f64, f64, f64, f64) {
    let t6201 = t5672 * t6199 * t446;
    let t6205 = t1515 * t1516 * t1392;
    let t6210 = t221 * t1475 * t1430;
    let t6213 = t209 * t589;
    let t6215 = t221 * t1501 * t6213;
    (t6201, t6205, t6210, t6215)
}
