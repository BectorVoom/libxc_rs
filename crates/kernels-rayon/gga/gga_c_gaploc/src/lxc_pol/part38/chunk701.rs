//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 701/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk701(t13433: f64, t1445: f64, t4527: f64, t11408: f64, t874: f64, t1562: f64, t3377: f64, t3566: f64, t11362: f64, t13296: f64, t189: f64, t188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13434 = t1445 * t13433;
    let t13436 = 0.27606906686822939767e2_f64 * t4527 * t13434;
    let t13437 = t11408 * t874;
    let t13438 = t1445 * t13437;
    let t13440 = 0.69017266717057349418e1_f64 * t1562 * t13438;
    let t13442 = 0.25025342966295298669e1_f64 * t3566 * t3377;
    let t13444 = 0.10725146985555128001e1_f64 * t11362 * t3377;
    let t13445 = t189 * t13296;
    let t13446 = t188 * t13445;
    (t13434, t13436, t13437, t13438, t13440, t13442, t13444, t13445, t13446)
}
