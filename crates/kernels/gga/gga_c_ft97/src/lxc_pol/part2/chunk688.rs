//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 688/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk688<F: Float>(t12118: F, t12122: F, t2294: F, t3640: F, t637: F, t2253: F, t3642: F, t11034: F, t3613: F, t1736: F, t179: F, t11008: F, t12099: F, t12102: F, t12104: F, t12108: F, t12113: F, t12119: F, t2265: F, t631: F, t8641: F, t8643: F, t8645: F, t8647: F, t8676: F, t8678: F, t8714: F, t8718: F, t8719: F) -> (F,) {
    let t12123 = t12122 * t12118;
    let t12128 = t637 * t3640 * t2294;
    let t12132 = 2.0 * t2253 * t3642;
    let t12134 = t3613 * t11034;
    let t12137 = t1736 * t179;
    let t12138 = t12137 * t11008;
    let t12141 = 10.0 / 27.0 * t8641 - t8643 / 9.0 - t8645 / 27.0 + 2.0 / 3.0 * t2265 * t12099 + t2265 * t12102 - t2265 * t12104 / 3.0 + t2265 * t12108 + t8647 - t8714 / 3.0 + 10.0 / 9.0 * t8719 + 2.0 * t2265 * t12113 + 4.0 / 3.0 * t2265 * t12119 - 2.0 / 9.0 * t2265 * t12123 + 4.0 / 9.0 * t8676 + t8718 - 3.0 / 2.0 * t631 * t12128 + t12132 + 2.0 / 9.0 * t8678 + t2265 * t12134 / 18.0 + 2.0 / 27.0 * t2265 * t12138;
    (t12141,)
}
