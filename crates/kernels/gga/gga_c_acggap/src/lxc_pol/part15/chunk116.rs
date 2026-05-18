//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 116/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk116<F: Float>(t1: F, t203: F, t355: F, t136: F, t352: F, t344: F, t348: F) -> (F, F, F, F) {
    let t357 = t355 * t1 * t203;
    let t358 = t352 * t136 * t357;
    let t359 = t358 / F::new(24.0);
    let t360 = -t344 - t348 / F::new(4.0) + t359;
    (t357, t358, t359, t360)
}
