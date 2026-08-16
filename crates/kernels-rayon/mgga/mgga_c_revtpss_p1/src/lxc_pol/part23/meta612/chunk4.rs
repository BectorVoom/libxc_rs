//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2282/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2282(t1161: f64, t1180: f64, t12429: f64, t12470: f64, t12486: f64, t12553: f64, t17097: f64, t1745: f64, t1757: f64, t20526: f64, t20542: f64, t24214: f64, t24217: f64, t24331: f64, t24363: f64, t24366: f64, t24376: f64, t24408: f64, t24411: f64, t24414: f64, t24417: f64, t24420: f64, t24423: f64, t3452: f64, t3477: f64, t3496: f64, t3521: f64, t5158: f64, t6535: f64, t6538: f64) -> f64 {
    let t24428 = -0.19298375398431042081e3_f64 * t12429 * t24331 + 1.0_f64 * t1161 * t24363 + 0.2069040516770936012e4_f64 * t12470 * t24366 + 0.17544670867903938621e1_f64 * t20526 * t1757 + 0.17544670867903938621e1_f64 * t5158 * t6535 + 0.51947577317044391276e2_f64 * t17097 * t6538 - 0.10389515463408878255e3_f64 * t12486 * t24376 + 0.5848223622634646207e0_f64 * t1180 * t24408 + 0.10254018858216406658e4_f64 * t12553 * t24411 + 0.51947577317044391277e2_f64 * t3521 * t24414 + t24214 - t24217 - 6.0_f64 * t3452 * t24417 + 0.96491876992155210402e2_f64 * t3477 * t24420 - 0.35089341735807877242e1_f64 * t3496 * t24423 + 3.0_f64 * t20542 * t1745;
    t24428
}
