//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 591/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk591<F: Float>(t105: F, t3124: F, t3132: F, t3329: F, t3341: F, t3346: F, t3349: F, t3353: F, t3357: F, t3359: F, t209: F, t2798: F, t921: F) -> (F, F, F) {
    let t3362 = t3329 + F::new(0.28455006635676149599e-1) * t105 * t3341 + t3346 - t3349 + t3124 - t3132 - t3353 + t3357 - F::new(0.28455006635676149599e-1) * t105 * t3359;
    let t3363 = t3362 * t209;
    let t3364 = t2798 * t921;
    (t3362, t3363, t3364)
}
