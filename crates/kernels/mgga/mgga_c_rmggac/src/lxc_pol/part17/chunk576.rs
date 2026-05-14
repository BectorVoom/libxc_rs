//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 576/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk576<F: Float>(t2392: F, t352: F, t262: F, t8620: F, t2350: F, t321: F, t7198: F, t22: F, t3924: F, t235: F, t333: F, t7192: F, t511: F, t899: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8621 = t2392 * t352;
    let t8622 = t262 * t8621;
    let t8623 = t8620 * t8622;
    let t8625 = t2350 * t321;
    let t8626 = t262 * t8625;
    let t8627 = t7198 * t8626;
    let t8629 = t3924 * t22;
    let t8630 = t235 * t8629;
    let t8631 = t2350 * t333;
    let t8632 = t262 * t8631;
    let t8633 = t8630 * t8632;
    let t8635 = t2350 * t352;
    let t8636 = t262 * t8635;
    let t8637 = t7192 * t8636;
    let t8639 = t511 * t22;
    let t8640 = t899 * t8639;
    (t8621, t8622, t8623, t8625, t8626, t8627, t8629, t8630, t8631, t8632, t8633, t8635, t8636, t8637, t8639, t8640)
}
