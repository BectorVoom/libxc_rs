//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 536/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk536(t701: f64, t9603: f64, t2580: f64, t3270: f64, t702: f64, t3236: f64, t779: f64, t1987: f64, t3276: f64, t3248: f64, t731: f64, t3240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9604 = t9603 * t701;
    let t9605 = t2580 * t9604;
    let t9608 = t3270 * t702;
    let t9611 = t779 * t3236;
    let t9614 = t3276 * t1987;
    let t9618 = 0.85450291446024714264e-3_f64 * t731 * t3248;
    let t9620 = 0.85450291446024714264e-3_f64 * t731 * t3240;
    (t9604, t9605, t9608, t9611, t9614, t9618, t9620)
}
