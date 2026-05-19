//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 229/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk229<F: Float>(t62: F, t774: F, t755: F, t752: F, t684: F, t688: F, t707: F, t712: F, t750: F, t82: F, t165: F, t164: F) -> (F, F, F, F, F, F) {
    let t775 = t62 * t774;
    let t776 = t755 * t775;
    let t777 = t752 * t776;
    let t779 = t684 * t82 - F::new(0.66725e-1) * t688 * t707 - F::cast_from(0.13265555555555555555e-1_f64) * t712 + F::cast_from(0.99491666666666666664e-2_f64) * t750 - F::cast_from(0.99491666666666666664e-2_f64) * t777;
    let t780 = t779 * t165;
    let t781 = t164 * t164;
    (t775, t776, t777, t779, t780, t781)
}
