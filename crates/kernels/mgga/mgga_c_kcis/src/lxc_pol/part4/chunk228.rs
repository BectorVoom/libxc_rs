//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 228/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk228<F: Float>(t62: F, t774: F, t755: F, t752: F, t684: F, t688: F, t707: F, t712: F, t750: F, t82: F, t165: F, t164: F, t142: F, t143: F, t126: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t775 = t62 * t774;
    let t776 = t755 * t775;
    let t777 = t752 * t776;
    let t779 = t684 * t82 - 0.66725e-1 * t688 * t707 - 0.13265555555555555555e-1 * t712 + 0.99491666666666666664e-2 * t750 - 0.99491666666666666664e-2 * t777;
    let t780 = t779 * t165;
    let t781 = t164 * t164;
    let t782 = 1.0 / t781;
    let t783 = t142 * t782;
    let t784 = t684 * t143;
    let t787 = t60 * t126;
    (t775, t776, t777, t779, t780, t781, t782, t783, t784, t787)
}
