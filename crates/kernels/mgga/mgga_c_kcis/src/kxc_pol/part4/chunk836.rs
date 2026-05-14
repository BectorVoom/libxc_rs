//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 836/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk836<F: Float>(t449: F, t6260: F, t446: F, t4529: F, t113: F, t774: F, t2150: F, t62: F, t822: F, t251: F, t4863: F, t2532: F, t2537: F, t779: F, t2539: F, t2728: F, t887: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6261 = t449 * t6260;
    let t6262 = t446 * t6261;
    let t6292 = 2.0 * t4529;
    let t7617 = t113 * t774;
    let t7624 = t2150 * t774;
    let t7627 = t62 * t822;
    let t8291 = t251 * t4863;
    let t8521 = 3.0 * t2532;
    let t8522 = t779 * t2537;
    let t8523 = t8522 * t2539;
    let t8524 = 6.0 * t8523;
    let t8525 = t887 * t2728;
    (t6262, t6292, t7617, t7624, t7627, t8291, t8521, t8524, t8525)
}
