//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 978/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk978<F: Float>(t12555: F, t24375: F, t1756: F, t20671: F, t1745: F, t6502: F, t1744: F, t20618: F, t1757: F, t6534: F, t1161: F, t1180: F, t12429: F, t12470: F, t12486: F, t12553: F, t17097: F, t20526: F, t20542: F, t24214: F, t24217: F, t24331: F, t24363: F, t24366: F, t24376: F, t24408: F, t3452: F, t3477: F, t3496: F, t3521: F, t5158: F, t6535: F, t6538: F) -> (F,) {
    let t24411 = t24375 * t12555;
    let t24414 = t20671 * t1756;
    let t24417 = t1745 * t6502;
    let t24420 = t20618 * t1744;
    let t24423 = t1757 * t6534;
    let t24428 = -0.19298375398431042081e3 * t12429 * t24331 + 1.0 * t1161 * t24363 + 0.2069040516770936012e4 * t12470 * t24366 + 0.17544670867903938621e1 * t20526 * t1757 + 0.17544670867903938621e1 * t5158 * t6535 + 0.51947577317044391276e2 * t17097 * t6538 - 0.10389515463408878255e3 * t12486 * t24376 + 0.5848223622634646207e0 * t1180 * t24408 + 0.10254018858216406658e4 * t12553 * t24411 + 0.51947577317044391277e2 * t3521 * t24414 + t24214 - t24217 - 6.0 * t3452 * t24417 + 0.96491876992155210402e2 * t3477 * t24420 - 0.35089341735807877242e1 * t3496 * t24423 + 3.0 * t20542 * t1745;
    (t24428,)
}
