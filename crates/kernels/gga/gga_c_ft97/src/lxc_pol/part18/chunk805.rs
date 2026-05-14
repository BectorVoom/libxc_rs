//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 805/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk805<F: Float>(t22632: F, t5598: F, t5599: F, t7187: F, t11: F, t14: F, t5592: F, t172: F, t5590: F) -> (F, F, F, F, F, F) {
    let t22634 = t5598 * t22632 * t5599;
    let t22636 = 1.0 / t7187;
    let t22637 = t11 * t22636;
    let t22638 = t22637 * t14;
    let t22639 = t22638 * t5592;
    let t22642 = t5590 * t172;
    (t22634, t22636, t22637, t22638, t22639, t22642)
}
