//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2359/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2359<F: Float>(t1121: F, t1248: F, t606: F, t3604: F, t17353: F, t372: F, t5277: F) -> (F, F, F, F) {
    let t17655 = t1248 * t1121;
    let t17656 = t17655 * t606;
    let t17657 = t3604 * t17656;
    let t17658 = t17353 * t17657;
    let t17661 = t372 * t5277;
    (t17656, t17657, t17658, t17661)
}
