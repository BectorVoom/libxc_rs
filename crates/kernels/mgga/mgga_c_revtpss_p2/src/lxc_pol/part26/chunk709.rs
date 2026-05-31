//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 709/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk709<F: Float>(t3: F, t7541: F, t116: F, t2055: F, t670: F, t117: F, t7373: F, t1459: F, t1461: F, t2113: F, t2115: F, t572: F, t573: F, param_d: F) -> (F, F, F, F, F, F) {
    let t7542 = t3 * t7541;
    let t7547 = param_d * t7541;
    let t7553 = t116 * t2055;
    let t7554 = t7553 * t670;
    let t7557 = t117 * t7373;
    let t7560 = F::cast_from(3.0_f64) * t1459 * t2115 + F::cast_from(3.0_f64) * t1461 * t2113 + F::cast_from(6.0_f64) * t572 * t7554 + F::cast_from(3.0_f64) * t572 * t7557 + t573 * t7547;
    (t7542, t7547, t7553, t7554, t7557, t7560)
}
