//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 538/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk538(t2293: f64, t2334: f64, t1445: f64, t9219: f64, t203: f64, t3085: f64, t447: f64, t1457: f64, t9215: f64, t9211: f64, t3158: f64, t528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9309 = t2334 * t2293;
    let t9310 = t1445 * t9309;
    let t9313 = t1445 * t9219;
    let t9316 = t203 * t3085;
    let t9317 = t9316 * t447;
    let t9318 = t1445 * t9317;
    let t9321 = t1457 * t9219;
    let t9324 = t1457 * t9215;
    let t9327 = t1457 * t9211;
    let t9330 = t528 * t3158;
    (t9310, t9313, t9316, t9318, t9321, t9324, t9327, t9330)
}
