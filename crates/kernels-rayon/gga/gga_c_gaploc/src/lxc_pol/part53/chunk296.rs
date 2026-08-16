//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 296/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk296(t2389: f64, t898: f64, t1457: f64, t2345: f64, t1445: f64, t2335: f64, t2344: f64, t447: f64, t528: f64, t894: f64, t1: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2390 = t898 * t2389;
    let t2392 = t1457 * t2345;
    let t2395 = t1445 * t2335;
    let t2398 = t2344 * t447;
    let t2399 = t1445 * t2398;
    let t2402 = t528 * t894;
    let t2405 = t874 * t1;
    (t2390, t2392, t2395, t2399, t2402, t2405)
}
