//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1620/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1620<F: Float>(t13142: F, t17708: F, t3601: F, t3603: F, t17710: F, t3720: F, t13127: F, t471: F, t17730: F, t5046: F, t12787: F, t1260: F, t5261: F) -> (F, F, F, F, F, F) {
    let t17747 = t13142 * t17708;
    let t17748 = t3601 * t3603;
    let t17749 = t17710 * t17748;
    let t17750 = t3720 * t17749;
    let t17753 = t13127 * t17708;
    let t17754 = t3601 * t471;
    let t17755 = t17710 * t17754;
    let t17756 = t3720 * t17755;
    let t17759 = t5046 * t17730;
    let t17760 = t12787 * t17759;
    let t17763 = t5261 * t1260;
    (t17747, t17750, t17753, t17756, t17760, t17763)
}
