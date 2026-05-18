//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1049/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1049<F: Float>(t1161: F, t1180: F, t12429: F, t12470: F, t12486: F, t12553: F, t17097: F, t1745: F, t1757: F, t20526: F, t20542: F, t24214: F, t24217: F, t24331: F, t24363: F, t24366: F, t24376: F, t24408: F, t24411: F, t24414: F, t24417: F, t24420: F, t24423: F, t3452: F, t3477: F, t3496: F, t3521: F, t5158: F, t6535: F, t6538: F) -> F {
    let t24428 = -F::new(0.19298375398431042081e3) * t12429 * t24331 + F::new(1.0) * t1161 * t24363 + F::new(0.2069040516770936012e4) * t12470 * t24366 + F::new(0.17544670867903938621e1) * t20526 * t1757 + F::new(0.17544670867903938621e1) * t5158 * t6535 + F::new(0.51947577317044391276e2) * t17097 * t6538 - F::new(0.10389515463408878255e3) * t12486 * t24376 + F::new(0.5848223622634646207e0) * t1180 * t24408 + F::new(0.10254018858216406658e4) * t12553 * t24411 + F::new(0.51947577317044391277e2) * t3521 * t24414 + t24214 - t24217 - F::new(6.0) * t3452 * t24417 + F::new(0.96491876992155210402e2) * t3477 * t24420 - F::new(0.35089341735807877242e1) * t3496 * t24423 + F::new(3.0) * t20542 * t1745;
    t24428
}
