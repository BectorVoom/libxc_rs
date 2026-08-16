//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3808/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3808<F: Float>(t68971: F, t69030: F, t69090: F, t69094: F, t69097: F, t69099: F, t69101: F, t69103: F, t69105: F, t69107: F, t69111: F, t69115: F, t69117: F, t69599: F) -> F {
    let t73285 = -t68971 + t69030 - t69090 + t69094 - t69097 + t69099 + t69101 - t69103 + t69105 + t69107 - t69111 - t69115 + t69117 + t69599;
    t73285
}
