//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2374/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2374<F: Float>(t3601: F, t3603: F, t17710: F, t3720: F, t13127: F, t17708: F) -> (F, F, F, F) {
    let t17748 = t3601 * t3603;
    let t17749 = t17710 * t17748;
    let t17750 = t3720 * t17749;
    let t17753 = t13127 * t17708;
    (t17748, t17749, t17750, t17753)
}
