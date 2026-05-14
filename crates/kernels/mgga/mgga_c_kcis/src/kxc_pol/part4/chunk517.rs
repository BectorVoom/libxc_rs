//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 517/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk517<F: Float>(t2318: F, t2321: F, t2323: F, t2327: F, t2329: F, t2331: F, t851: F, t843: F, t189: F, t197: F, t2665: F, t673: F, t88: F, t2354: F, t47: F, t2355: F, t680: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2675 = -0.78438333333333333333e0 * t2318 + 0.15687666666666666667e1 * t2321 + 0.68863333333333333333e0 * t2323 + 0.14025833333333333333e0 * t2327 + 0.28051666666666666667e0 * t2329 + 0.17365833333333333333e0 * t2331;
    let t2676 = t2675 * t851;
    let t2679 = t843 * t843;
    let t2680 = 1.0 / t2679;
    let t2681 = t189 * t2680;
    let t2682 = t197 * t197;
    let t2683 = 1.0 / t2682;
    let t2684 = t2665 * t2683;
    let t2690 = t88 * t673;
    let t2694 = t47 * t2354;
    let t2695 = t2355 * t680;
    (t2675, t2676, t2679, t2680, t2681, t2682, t2683, t2684, t2690, t2694, t2695)
}
