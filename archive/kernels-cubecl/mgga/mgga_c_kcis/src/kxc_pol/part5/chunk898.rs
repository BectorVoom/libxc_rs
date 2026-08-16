//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 898/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk898<F: Float>(t449: F, t7570: F, t446: F, t113: F, t774: F, t2150: F, t62: F, t822: F, t251: F, t4863: F, t2532: F, t2537: F, t779: F) -> (F, F, F, F, F, F, F) {
    let t7571 = t449 * t7570;
    let t7572 = t446 * t7571;
    let t7573 = t7572 / F::cast_from(16.0_f64);
    let t7617 = t113 * t774;
    let t7624 = t2150 * t774;
    let t7627 = t62 * t822;
    let t8291 = t251 * t4863;
    let t8521 = F::cast_from(3.0_f64) * t2532;
    let t8522 = t779 * t2537;
    (t7573, t7617, t7624, t7627, t8291, t8521, t8522)
}
