//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 966/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk966(t435: f64, t5079: f64, t13080: f64, t4925: f64, t12747: f64, t1466: f64, t1165: f64, t3194: f64, t4289: f64, t5284: f64, t14176: f64, t4967: f64) -> (f64, f64, f64, f64, f64) {
    let t15565 = t435 * t5079;
    let t15574 = t13080 * t4925;
    let t15576 = t12747 * t1466;
    let t15610 = t3194 * t1165 * t4289 * t5284;
    let t15622 = t14176 * t4967;
    (t15565, t15574, t15576, t15610, t15622)
}
