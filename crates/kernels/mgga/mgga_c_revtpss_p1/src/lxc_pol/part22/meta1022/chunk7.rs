//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3568/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3568<F: Float>(t20230: F, t3336: F, t1100: F, t1102: F, t198: F, t336: F, t5023: F, t64467: F, t64471: F, t64475: F, t64483: F, t64567: F, t64592: F, t64626: F, t64661: F, t64694: F, t64722: F, t64753: F, t64788: F, t64822: F, t65402: F, t65404: F, t65408: F, t65413: F, t65415: F, t65417: F, t65419: F, t65421: F, t68006: F, t68038: F, t68067: F, t68097: F, t68130: F, t68163: F, t68199: F) -> F {
    let t68207 = t20230 * t3336;
    let t68211 = t198 * t336 * (t64567 + t64592 + t64626 + t64661 + t64694 + t64722 + t64753 + t64788 + t64822 + t68006 + t68038 + t68067 + t68097 + t68130 + t68163 + t68199) * t1102 + t65402 + t64467 - t65404 + t65408 - F::new(2.0) * t5023 * t68207 * t1100 - t65413 + t65415 - t65417 - t65419 - t65421 + t64471 + t64475 + t64483;
    t68211
}
