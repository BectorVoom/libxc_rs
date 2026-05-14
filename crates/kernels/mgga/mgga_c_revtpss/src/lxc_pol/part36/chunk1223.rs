//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1223/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1223<F: Float>(t111453: F, t111516: F, t111537: F, t111675: F, t114288: F, t28154: F, t29388: F, t29412: F, t29538: F, t29544: F, t30683: F, t30686: F, t30689: F, t7566: F, t7706: F, t7709: F, t8144: F, t8147: F) -> (F,) {
    let t116844 = -5.0 * t28154 * t111675 + 5.0 * t111537 * t7706 + 2.0 * t29538 * t8144 + 5.0 * t29388 * t29544 + 2.0 * t29538 * t8147 + 5.0 / 2.0 * t111516 * t7706 + t7709 * t30683 + 5.0 * t29412 * t29544 + 2.0 * t7709 * t30686 + 5.0 / 2.0 * t7566 * t114288 + t7709 * t30689 - 5.0 * t111453 * t7706;
    (t116844,)
}
