//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1107/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1107(t2679: f64, t47168: f64, t9805: f64, t1991: f64, t47130: f64, t590: f64, t739: f64, t43413: f64, t43414: f64, t43417: f64, t43421: f64, t43426: f64, t47155: f64, t47157: f64, t47160: f64, t47164: f64, t47166: f64) -> f64 {
    let t47170 = t9805 * t47168 * t2679;
    let t47174 = t1991 * t739 * t47130 * t590;
    let t47176 = -t43413 + t43414 - t43417 + t47155 - 0.57514388930881124514e0_f64 * t43421 + t47157 + t47160 - t47164 + 0.9585731488480187419e0_f64 * t47166 - 0.57514388930881124514e0_f64 * t47170 + 0.1022478025437886658e1_f64 * t47174 - t43426;
    t47176
}
