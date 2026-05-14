//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 858/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk858<F: Float>(t22671: F, t36: F, t70: F, t1486: F, t5826: F, t1470: F, t5854: F, t1469: F, t5819: F) -> (F, F, F, F, F) {
    let t22672 = t36 * t22671;
    let t22673 = t22672 * t70;
    let t22676 = t5826 * t1486;
    let t22681 = t1470 * t5854;
    let t22688 = t5819 * t1469;
    (t22672, t22673, t22676, t22681, t22688)
}
