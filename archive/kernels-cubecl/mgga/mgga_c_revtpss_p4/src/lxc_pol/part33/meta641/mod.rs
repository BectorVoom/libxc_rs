//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta641<F: Float>(t28150: F, t7575: F, t101214: F, t2122: F, t60221: F, t7565: F, t13272: F, t26754: F, t25163: F, t8143: F, t101226: F, t101218: F) -> (F, F, F, F, F, F, F) {
        let (t104215, t104226, t104279, t104282, t104314, t104317, t104332) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2090::<F>(t28150, t7575, t101214, t2122, t60221, t7565, t13272, t26754, t25163, t8143, t101226, t101218);
    (t104215, t104226, t104279, t104282, t104314, t104317, t104332)
}
