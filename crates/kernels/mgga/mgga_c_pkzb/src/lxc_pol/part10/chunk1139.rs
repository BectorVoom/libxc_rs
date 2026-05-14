//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1139/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1139<F: Float>(t173: F, t5286: F, t588: F, t603: F, t158: F, t165: F, t5387: F, t1721: F, t1760: F, t5384: F, t1756: F, t1511: F, t5331: F, t1613: F, t4952: F, t542: F, t555: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16405 = t5286 * t173;
    let t16406 = t588 * t16405;
    let t16407 = t16406 * t603;
    let t16421 = t158 / t5387 / t165;
    let t16425 = t1721 * t1721;
    let t16440 = t5384 * t1760;
    let t16467 = t5384 * t1756;
    let t16476 = t1511 * t5331;
    let t16481 = 0.46785788981077169656e1 * t555 * t1613 * t4952 * t542;
    (t16405, t16406, t16407, t16421, t16425, t16440, t16467, t16476, t16481)
}
