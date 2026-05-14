//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 840/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk840<F: Float>(t7572: F, t113: F, t774: F, t2150: F, t62: F, t822: F, t251: F, t4863: F, t2532: F, t2537: F, t779: F, t2539: F, t2728: F, t887: F, t2751: F, t2489: F, t747: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7573 = t7572 / 16.0;
    let t7617 = t113 * t774;
    let t7624 = t2150 * t774;
    let t7627 = t62 * t822;
    let t8291 = t251 * t4863;
    let t8521 = 3.0 * t2532;
    let t8522 = t779 * t2537;
    let t8523 = t8522 * t2539;
    let t8524 = 6.0 * t8523;
    let t8525 = t887 * t2728;
    let t8526 = t8525 * t2751;
    let t8531 = t747 * t2489;
    (t7573, t7617, t7624, t7627, t8291, t8521, t8524, t8526, t8531)
}
