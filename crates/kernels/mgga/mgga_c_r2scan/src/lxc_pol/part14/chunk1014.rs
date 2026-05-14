//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1014/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1014<F: Float>(t38152: F, t7418: F, t38149: F, t39469: F, t11780: F, t2207: F, t3328: F, t11793: F, t2201: F, t3336: F, t2833: F, t545: F, t3300: F, t10875: F, t11744: F, t146: F, t2206: F, t2832: F) -> (F, F, F, F, F, F, F) {
    let t39721 = t38152 * t7418;
    let t39723 = t38149 * t39469;
    let t39727 = t2207 * t11780 * t3328;
    let t39730 = t2201 * t3336 * t11793;
    let t39739 = t545 * t2833;
    let t39740 = t39739 * t3300;
    let t39742 = t11744 * t10875;
    let t39745 = t146 * t2206 * t2832;
    (t39721, t39723, t39727, t39730, t39740, t39742, t39745)
}
