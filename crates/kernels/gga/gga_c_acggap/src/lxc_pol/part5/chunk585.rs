//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 585/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk585<F: Float>(t1095: F, t3529: F, t398: F, t384: F, t360: F, t944: F, t372: F, t177: F, t414: F, t980: F, t378: F, t968: F) -> (F, F, F, F, F, F) {
    let t3531 = t398 * t1095 * t3529;
    let t3532 = t384 * t3531;
    let t3539 = t944 * t360;
    let t3544 = t944 * t372;
    let t3551 = F::cast_from(0.30011812682648815881e-2_f64) * t980 * t414 * t177;
    let t3552 = t378 * t968;
    (t3531, t3532, t3539, t3544, t3551, t3552)
}
