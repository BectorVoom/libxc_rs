//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1132/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1132<F: Float>(t10764: F, t26282: F, t10882: F, t11748: F, t38152: F, t7418: F, t38149: F, t39469: F, t11780: F, t2207: F, t3328: F, t11793: F, t2201: F, t3336: F) -> (F, F, F, F, F, F) {
    let t39717 = t26282 * t10764;
    let t39719 = t11748 * t10882;
    let t39721 = t38152 * t7418;
    let t39723 = t38149 * t39469;
    let t39727 = t2207 * t11780 * t3328;
    let t39730 = t2201 * t3336 * t11793;
    (t39717, t39719, t39721, t39723, t39727, t39730)
}
