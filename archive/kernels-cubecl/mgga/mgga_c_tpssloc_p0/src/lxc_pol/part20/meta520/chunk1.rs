//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2049/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2049<F: Float>(t17: F, t2516: F, t3681: F, t12126: F, t592: F, t1287: F, t9218: F, t118: F, t142: F, t39283: F) -> (F, F, F, F) {
    let t39642 = t17 * t3681 * t2516;
    let t39644 = t592 * t12126;
    let t39655 = F::cast_from(480.0_f64) * t9218 * t1287;
    let t39658 = F::cast_from(0.11483599538271604938e-1_f64) * t118 * t39283 * t142;
    (t39642, t39644, t39655, t39658)
}
