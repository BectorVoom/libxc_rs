//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 737/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk737(t1365: f64, t31: f64, t4: f64, t1230: f64, t1253: f64, t1259: f64, t1304: f64, t1320: f64, t440: f64, t442: f64, t450: f64, t4503: f64, t4506: f64, t4513: f64, t4539: f64, t4542: f64, t4606: f64, t4608: f64, t4620: f64, t4624: f64, t4631: f64, t4636: f64, t4637: f64, t4640: f64, t4643: f64, t71: f64, t84: f64) -> (f64, f64) {
    let t4651 = t4 * t1365 * t31;
    let t4652 = 0.34451131037037037036e-2_f64 * t4651;
    let t4656 = -t4503 + t4506 + t4513 - t4539 - t4542 - 0.1038945353962551798e3_f64 * t4606 * t4608 + 0.58482233974552040708e0_f64 * t450 * t4620 + 0.51947267698127589897e2_f64 * t1320 * t4624 - 6.0_f64 * t1230 * t442 * t1253 + 6.0_f64 * t1259 * t4631 - 0.19298809906722418785e3_f64 * t4636 * t4637 - 0.35089340384731224426e1_f64 * t1304 * t4640 + 0.96494049533612093922e2_f64 * t1259 * t4643 * t440 + 0.56969282336565386482e-3_f64 * t4 * t1365 * t84 - t4652 + 0.16562449037037037036e-2_f64 * t4 * t1365 * t71;
    (t4652, t4656)
}
