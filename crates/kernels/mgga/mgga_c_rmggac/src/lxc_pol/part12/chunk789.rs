//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 789/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk789<F: Float>(t4601: F, t7769: F, t275: F, t7950: F, t132: F, t26078: F, t36: F, t4787: F, t638: F, t71: F, t2127: F, t934: F) -> (F, F, F, F) {
    let t36680 = t4601 * t7769;
    let t36689 = t275 * t7950;
    let t36700 = t638 * t36 * t26078 * t71 * t132 * t4787;
    let t36701 = F::new(0.91462949374725084942e-3) * t36700;
    let t36710 = t934 * t2127;
    (t36680, t36689, t36701, t36710)
}
