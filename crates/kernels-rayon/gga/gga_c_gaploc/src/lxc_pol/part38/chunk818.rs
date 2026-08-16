//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 818/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk818(t13058: f64, t1991: f64, t20671: f64, t28309: f64, t33601: f64, t33565: f64, t7372: f64, t33294: f64, t9810: f64, t10667: f64, t123: f64, t883: f64) -> (f64, f64, f64, f64, f64) {
    let t43657 = t1991 * t13058;
    let t43660 = t28309 * t20671 * t33601;
    let t43679 = t33565 * t7372;
    let t43681 = t33294 * t9810;
    let t43710 = t10667 * t123 * t883;
    (t43657, t43660, t43679, t43681, t43710)
}
