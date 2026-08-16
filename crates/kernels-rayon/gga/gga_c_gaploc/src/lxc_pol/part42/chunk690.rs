//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 690/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk690(t13387: f64, t1429: f64, t11426: f64, t6590: f64, t3516: f64, t6508: f64, t2365: f64, t4391: f64, t123: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13388 = t1429 * t13387;
    let t13389 = 0.14896037479937677779e-1_f64 * t13388;
    let t13390 = t11426 * t6590;
    let t13392 = t6508 * t3516;
    let t13393 = t2365 * t13392;
    let t13394 = t4391 * t13393;
    let t13395 = 0.29792074959875355558e-1_f64 * t13394;
    let t13396 = t3516 * t123;
    let t13397 = t13396 * t883;
    (t13389, t13390, t13392, t13393, t13395, t13396, t13397)
}
