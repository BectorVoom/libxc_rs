//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1077/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1077<F: Float>(t39785: F, t39792: F, t37883: F, t37891: F, t37893: F, t37903: F, t37905: F, t39789: F, t39795: F, t39801: F, t39804: F, t39807: F, t39816: F, t39823: F, t39825: F, t39827: F) -> (F, F, F, F, F) {
    let t41552 = 0.39029762157531132074e-1 * t39785;
    let t41555 = 0.46230515946956099004e0 * t39792;
    let t41564 = t41552 - 0.32927245914677557992e-1 * t37883 - 0.5200933044032561138e1 * t39789 + t41555 - 0.52396431978519890152e-1 * t39795 - 0.17073386770573548589e1 * t37891 + 0.25610080155860322884e0 * t37893 - 0.62295486109113302474e-1 * t37903 - 0.47609969197673950973e-2 * t37905 + 0.43663693315433241794e-2 * t39801 + 0.17336443480108537126e0 * t39804 + 0.2600466522016280569e0 * t39807;
    let t41570 = 0.11902492299418487743e0 * t39816;
    let t41573 = 0.95219938395347901946e-2 * t39823;
    let t41574 = 0.28565981518604370584e-1 * t39825;
    let t41575 = 0.95219938395347901946e-2 * t39827;
    (t41564, t41570, t41573, t41574, t41575)
}
