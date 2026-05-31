//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 703/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk703<F: Float>(t3555: F, t487: F, t1204: F, t1207: F, t458: F, t456: F) -> (F, F, F, F) {
    let t3556 = t3555 * t487;
    let t3561 = t1204 * t487;
    let t3565 = F::cast_from(1.0_f64) / t1207 / t458;
    let t3566 = t456 * t3565;
    (t3556, t3561, t3565, t3566)
}
