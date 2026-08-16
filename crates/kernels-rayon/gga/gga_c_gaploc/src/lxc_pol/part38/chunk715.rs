//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 715/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk715(t3684: f64, t977: f64, t1960: f64, t3601: f64, t7290: f64, t2365: f64, t6111: f64, t2610: f64, t3614: f64, t2033: f64, t11845: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13585 = t3684 * t977;
    let t13587 = 2.0_f64 * t1960 * t13585;
    let t13588 = t7290 * t3601;
    let t13589 = t2365 * t13588;
    let t13590 = t6111 * t13589;
    let t13591 = 0.29792074959875355558e-1_f64 * t13590;
    let t13592 = t2610 * t3614;
    let t13593 = t2365 * t13592;
    let t13594 = t2033 * t13593;
    let t13595 = 0.14896037479937677779e-1_f64 * t13594;
    let t13596 = t11845 * t959;
    (t13585, t13587, t13588, t13589, t13591, t13592, t13593, t13595, t13596)
}
