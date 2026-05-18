//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1370/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1370<F: Float>(t114322: F, t114343: F, t114349: F, t1923: F, t2122: F, t2123: F, t29513: F, t29532: F, t29551: F, t30683: F, t30686: F, t30689: F, t7702: F, t8143: F, t8144: F, t8147: F) -> F {
    let t116759 = -t1923 * t8143 * t29532 / F::new(2.0) - t1923 * t2122 * t114343 / F::new(6.0) + t29551 * t8144 + t29551 * t8147 + t114322 * t2123 - t114349 * t2123 / F::new(6.0) - t29513 * t8144 / F::new(2.0) - t29513 * t8147 / F::new(2.0) - t7702 * t30683 / F::new(2.0) - t7702 * t30686 - t7702 * t30689 / F::new(2.0);
    t116759
}
