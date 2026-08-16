//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3568/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3568(t20230: f64, t3336: f64, t1100: f64, t1102: f64, t198: f64, t336: f64, t5023: f64, t64467: f64, t64471: f64, t64475: f64, t64483: f64, t64567: f64, t64592: f64, t64626: f64, t64661: f64, t64694: f64, t64722: f64, t64753: f64, t64788: f64, t64822: f64, t65402: f64, t65404: f64, t65408: f64, t65413: f64, t65415: f64, t65417: f64, t65419: f64, t65421: f64, t68006: f64, t68038: f64, t68067: f64, t68097: f64, t68130: f64, t68163: f64, t68199: f64) -> f64 {
    let t68207 = t20230 * t3336;
    let t68211 = t198 * t336 * (t64567 + t64592 + t64626 + t64661 + t64694 + t64722 + t64753 + t64788 + t64822 + t68006 + t68038 + t68067 + t68097 + t68130 + t68163 + t68199) * t1102 + t65402 + t64467 - t65404 + t65408 - 2.0_f64 * t5023 * t68207 * t1100 - t65413 + t65415 - t65417 - t65419 - t65421 + t64471 + t64475 + t64483;
    t68211
}
