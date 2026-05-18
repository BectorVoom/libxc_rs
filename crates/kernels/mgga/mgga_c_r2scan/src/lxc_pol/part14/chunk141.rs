//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 141/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk141<F: Float>(t378: F, t381: F, t404: F, t432: F, t439: F, t447: F, t454: F, t5: F, t72: F, t85: F) -> F {
    let t457 = F::new(0.53237641966666666666e-3) * t5 * t378 * t72 + F::new(1.0) * t432 * t439 - t381 - t404 + F::new(0.18311447306006545054e-3) * t5 * t378 * t85 + F::new(0.5848223622634646207e0) * t447 * t454;
    t457
}
