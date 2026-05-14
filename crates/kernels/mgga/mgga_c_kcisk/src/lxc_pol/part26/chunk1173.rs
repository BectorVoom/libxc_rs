//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1173/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1173<F: Float>(t33883: F, t9536: F, t32153: F, t32436: F, t32474: F, t33493: F, t33496: F, t33499: F, t33501: F, t33510: F, t33784: F, t9519: F, t9855: F, t9860: F, t9864: F, t32459: F, t6480: F) -> (F, F, F) {
    let t33884 = t9536 * t33883;
    let t33900 = -0.5787037037037037037e-3 * t33884 + 0.11607361111111111111e-2 * t33493 + 0.11607361111111111111e-2 * t33496 - 0.17411041666666666666e-2 * t33499 - 0.10416666666666666667e-1 * t9536 * t33784 + 0.52083333333333333333e-2 * t9860 * t9519 + 0.20104166666666666667e-2 * t32474 * t9855 + 0.77382407407407407407e-3 * t33501 - 0.17361111111111111111e-2 * t32436 * t9864 + 0.11607361111111111111e-2 * t32153 - 0.11607361111111111111e-2 * t33510;
    let t33905 = t32459 * t6480;
    (t33884, t33900, t33905)
}
