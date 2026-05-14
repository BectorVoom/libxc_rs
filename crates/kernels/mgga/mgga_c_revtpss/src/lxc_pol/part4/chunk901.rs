//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 901/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk901<F: Float>(t123: F, t173: F, t186: F, t2537: F, t2548: F, t2554: F, t2556: F, t2597: F, t2604: F, t729: F, t731: F, t739: F, t9291: F, t9394: F, t9485: F, t9488: F, t9501: F, t9508: F, t9514: F, t9517: F, t9521: F, t9524: F, t9525: F, t9530: F, t9533: F, t9536: F, t9537: F, t9542: F) -> (F,) {
    let t9543 = 0.5848223622634646207e0 * t739 * t9485 + 0.35089341735807877242e1 * t2604 * t9488 + 0.16562821945185185185e-2 * t123 * t9291 * t173 - 6.0 * t2537 * t731 * t2548 + 0.96491876992155210402e2 * t2554 * t2548 * t2556 * t729 - 0.35089341735807877242e1 * t2597 * t9501 + 0.56968947174242584612e-3 * t123 * t9291 * t186 + 0.51947577317044391277e2 * t2604 * t9508 - t9394 - t9514 + t9517 + t9521 + t9524 + 6.0 * t2554 * t9525 + 0.2069040516770936012e4 * t9530 * t9533 + 0.10254018858216406658e4 * t9536 * t9537 - t9542;
    (t9543,)
}
