//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 482/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk482<F: Float>(t109: F, t287: F, t209: F, t421: F, t416: F, t25: F, t992: F) -> (F, F, F) {
    let t3495 = t109 * t287;
    let t3497 = t209 * t3495 * t421;
    let t3499 = t416 * t3497 / F::new(864.0);
    let t3500 = t25 * t992;
    (t3497, t3499, t3500)
}
