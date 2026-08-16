//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3122/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3122<F: Float>(t19262: F, t3640: F, t1164: F, t3400: F, t3403: F, t63283: F, t1156: F, t3375: F, t18276: F, t3411: F, t11126: F, t6106: F) -> (F, F, F, F, F) {
    let t64548 = t19262 * t3640;
    let t64558 = F::cast_from(0.34631718211362927518e2_f64) * t1164 * t3400 * t63283 * t3403;
    let t64562 = F::cast_from(0.23392894490538584828e1_f64) * t1164 * t3375 * t63283 * t1156;
    let t64564 = F::cast_from(0.20508037716432813316e4_f64) * t3411 * t18276;
    let t64566 = F::cast_from(0.17315859105681463759e2_f64) * t11126 * t6106;
    (t64548, t64558, t64562, t64564, t64566)
}
