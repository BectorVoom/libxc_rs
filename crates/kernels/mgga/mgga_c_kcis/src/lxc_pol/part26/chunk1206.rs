//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1206/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1206<F: Float>(t2398: F, t7639: F, t8759: F, t26490: F, t7633: F, t26450: F, t7647: F, t26477: F, t7636: F, t92016: F, t26501: F, t2155: F, t92055: F) -> (F, F, F, F, F, F, F, F) {
    let t92066 = t8759 * t2398 * t7639;
    let t92068 = t7633 * t26490;
    let t92070 = t26450 * t7647;
    let t92072 = t26450 * t7639;
    let t92074 = t7633 * t26477;
    let t92076 = t7636 * t92016;
    let t92078 = t7633 * t26501;
    let t92080 = t2155 * t92055;
    (t92066, t92068, t92070, t92072, t92074, t92076, t92078, t92080)
}
