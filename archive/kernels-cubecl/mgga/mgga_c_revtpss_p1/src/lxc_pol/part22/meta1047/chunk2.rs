//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3680/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3680<F: Float>(t300: F, t69192: F, t69216: F, t69383: F, t69422: F, t69467: F, t69500: F, t69548: F, t69595: F, t69090: F, t69094: F, t69097: F, t69099: F, t69101: F, t69103: F, t69105: F, t69107: F, t69111: F, t69115: F, t69117: F, t69569: F) -> (F, F) {
    let t69599 = t300 * (t69192 + t69216 + t69383 + t69422 + t69467 + t69500 + t69548 + t69595);
    let t69600 = -t69090 + t69094 - t69097 + t69099 + t69101 - t69103 + t69105 + t69107 - t69111 - t69115 + t69117 + t69599 + t69569;
    (t69599, t69600)
}
