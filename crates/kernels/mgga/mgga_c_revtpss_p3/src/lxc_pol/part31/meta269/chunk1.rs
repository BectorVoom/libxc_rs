//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1204/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1204<F: Float>(t670: F, t7330: F, t572: F, t117: F, t7002: F, t1461: F, t2040: F, t573: F, t7324: F, t7329: F, t38: F, t4173: F) -> (F, F, F, F) {
    let t7331 = t7330 * t670;
    let t7333 = F::cast_from(6.0_f64) * t572 * t7331;
    let t7334 = t117 * t7002;
    let t7336 = F::cast_from(3.0_f64) * t572 * t7334;
    let t7337 = F::cast_from(3.0_f64) * t1461 * t2040 + t573 * t7324 + t7329 + t7333 + t7336;
    let t7702 = t4173 * t38;
    (t7331, t7334, t7337, t7702)
}
