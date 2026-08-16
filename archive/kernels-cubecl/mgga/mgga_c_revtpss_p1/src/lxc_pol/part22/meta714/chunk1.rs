//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2746/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2746<F: Float>(t11028: F, t2439: F, t887: F, t11021: F, t2471: F, t2440: F, t2772: F, t10541: F, t2453: F, t10538: F, t231: F, t268: F, t2798: F, t793: F, t836: F) -> (F, F, F, F, F, F) {
    let t39565 = t2439 * t11028 * t887;
    let t39567 = t11021 * t2471;
    let t39573 = t2439 * t2440 * t2772;
    let t39575 = t2453 * t10541;
    let t39576 = t39575 * t10538;
    let t39581 = t2798 * t268 * t793 * t836 * t231;
    (t39565, t39567, t39573, t39575, t39576, t39581)
}
