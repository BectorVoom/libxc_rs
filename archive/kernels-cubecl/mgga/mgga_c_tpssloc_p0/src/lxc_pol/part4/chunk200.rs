//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 200/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk200<F: Float>(t592: F, t14: F, t2: F, t21: F, t15: F, t583: F, t19: F, t582: F, t586: F, t589: F, t83: F, t85: F) -> (F, F, F, F, F, F, F, F) {
    let t593 = F::cast_from(2.0_f64) * t592;
    let t594 = t14 * t2;
    let t596 = F::cast_from(0.1356e2_f64) * t594 * t21;
    let t597 = t15 * t583;
    let t598 = F::cast_from(1.0_f64) / t597;
    let t600 = F::cast_from(0.1356e2_f64) * t19 * t598;
    let t601 = t582 - t586 + t589 - t593 + t596 - t600;
    let t604 = F::cast_from(1.0_f64) / t85 / t83;
    (t593, t594, t596, t597, t598, t600, t601, t604)
}
