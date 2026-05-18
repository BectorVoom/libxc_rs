//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1166/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1166<F: Float>(t7589: F, t92187: F, t2410: F, t700: F, t706: F, t92184: F, t7580: F, t26602: F, t26623: F, t2389: F, t26620: F, t705: F) -> (F, F, F, F, F) {
    let t92188 = t7589 * t92187;
    let t92193 = t7589 * t92184 * t706 * t700 * t2410;
    let t92195 = t7580 * t92187;
    let t92197 = t26602 * t26623;
    let t92201 = t26620 * t2389 * t2410 * t705;
    (t92188, t92193, t92195, t92197, t92201)
}
