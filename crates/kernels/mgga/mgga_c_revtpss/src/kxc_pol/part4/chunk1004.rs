//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1004/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1004<F: Float>(t2516: F, t2523: F, t760: F, t9387: F, t2496: F, t9372: F, t37: F, t716: F, t2626: F, t9425: F, t2609: F, t606: F) -> (F, F, F, F, F, F, F, F) {
    let t10593 = t2523 * t2516;
    let t10596 = F::new(0.5848223622634646207e0) * t760 * t9387;
    let t10597 = t2523 * t2496;
    let t10604 = F::new(0.10254018858216406658e4) * t760 * t9372;
    let t10605 = t37 * t716;
    let t10608 = t2523 * t2626;
    let t10611 = F::new(0.35089341735807877242e1) * t760 * t9425;
    let t10612 = t2609 * t606;
    (t10593, t10596, t10597, t10604, t10605, t10608, t10611, t10612)
}
