//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 897/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk897(t43446: f64, t43454: f64, t2639: f64, t3614: f64, t7284: f64, t787: f64, t13593: f64, t5676: f64, t11576: f64, t2033: f64, t2365: f64, t2610: f64) -> (f64, f64, f64, f64, f64) {
    let t45287 = 0.41708904943825497782e0_f64 * t43446;
    let t45288 = 0.35750489951850426669e0_f64 * t43454;
    let t45298 = 0.25025342966295298669e1_f64 * t787 * t7284 * t3614 * t2639;
    let t45299 = t5676 * t13593;
    let t45300 = 0.14896037479937677779e-1_f64 * t45299;
    let t45303 = t2033 * t2365 * t2610 * t11576;
    (t45287, t45288, t45298, t45300, t45303)
}
