//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 763/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk763<F: Float>(t9459: F, t2737: F, t2740: F, t9464: F, t9467: F, t9471: F, t9476: F, t9479: F, t9512: F, t9516: F, t9519: F, t9524: F, t9529: F, t9534: F, t9536: F, t9539: F, t9544: F) -> (F, F) {
    let t9549 = 0.11607361111111111111e-2 * t9459;
    let t9555 = -0.52083333333333333333e-2 * t9512 * t2740 + 0.20104166666666666667e-2 * t9516 * t9519 - 0.52083333333333333333e-2 * t9524 * t2740 + 0.13888888888888888889e-1 * t9529 * t2740 - t9534 - 0.17361111111111111111e-2 * t9536 * t9539 + 0.52083333333333333333e-2 * t2737 * t9544 + 0.52083333333333333333e-2 * t2737 * t9519 + t9549 + 0.11607361111111111111e-2 * t9464 + 0.17411041666666666666e-2 * t9467 - 0.17411041666666666666e-2 * t9471 - 0.46429444444444444443e-2 * t9476 + 0.11607361111111111111e-2 * t9479;
    (t9549, t9555)
}
