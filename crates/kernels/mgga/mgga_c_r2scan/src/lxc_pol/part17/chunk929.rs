//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 929/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk929<F: Float>(t11357: F, t11566: F, t11570: F, t11580: F, t12721: F, t12723: F, t12726: F, t12728: F, t12730: F, t12733: F, t12939: F, t12942: F, t12962: F, t354: F, t1146: F, t3250: F) -> (F, F, F) {
    let t12964 = t12721 - 0.60975299583150056624e-3 * t11566 + 0.86737941314158990616e-4 * t11570 - t12723 - t12726 - t12728 - t12730 - t12733 - t11357 + 0.3842256877732895568e-2 * t11580 + t12939 + t12942 + t12962;
    let t12965 = t354 * t12964;
    let t12966 = t1146 * t3250;
    (t12964, t12965, t12966)
}
